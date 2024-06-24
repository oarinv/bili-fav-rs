use crate::read_cfg::read_config;
use fs_set_times::{set_mtime, SystemTimeSpec};
use reqwest::header;
use reqwest::header::COOKIE;
use reqwest::header::USER_AGENT;
use std::fs;
use std::process::Command;
use std::time::{Duration, SystemTime};
use std::{fs::File, io::Write};

pub async fn save_video(
    v_url: String,           // 视频下载链接
    a_url: String,           // 音频下载链接
    v_title: String,         // 视频下载标题，以m4s为结尾
    a_title: String,         // 音频下载标题，以ogg为结尾
    video_save_path: String, // 合并后的视频保存地址
    pubdate: u64,            // 视频上传时间
) -> Result<(), Box<dyn std::error::Error>> {
    // 传入音视频 URL 下载到文件夹中,如果视频没有音频则跳过音频,直接下载视频
    if a_url == "None" {
        download(v_url, v_title.clone()).await.expect("fail");
        ffmpeg_none_a_url(v_title, video_save_path.clone());
        update_file_change_time(pubdate, video_save_path.clone())
    } else {
        download(v_url, v_title.clone()).await.expect("fail");
        download(a_url, a_title.clone()).await.expect("fail");
        ffmpeg(a_title, v_title, video_save_path.clone());
        update_file_change_time(pubdate, video_save_path.clone())
    }

    Ok(())
}

async fn download(url: String, title: String) -> Result<(), Box<dyn std::error::Error>> {
    let user_agent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36";
    let url = format!("{}", url);

    let (s, _f, _fav_mode) = read_config();
    let cookie = format!("SESSDATA={}", s);
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(USER_AGENT, user_agent)
        .header(COOKIE, cookie)
        .header(header::REFERER, "https://www.bilibili.com")
        .send()
        .await?;

    if res.status().is_success() {
        // 打开一个文件用于写入下载的内容
        let filename = format!("{}", title);
        let mut file = File::create(filename)?;
        let bytes = res.bytes().await?;
        file.write_all(&bytes)?;
    } else {
        wd_log::log_warn_ln!("保存错误 {:?}", res.status());
    }

    Ok(())
}

// 通用ffmpeg合并视频
fn ffmpeg(a_title: String, v_title: String, video_save_path: String) {
    // 通过 FFmpeg 合并音视频文件为mp4文件
    let output = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(&v_title)
        .arg("-i")
        .arg(&a_title)
        .arg("-c")
        .arg("copy")
        .arg("-shortest")
        .arg(&video_save_path) // 合并输出目录和文件名
        .arg("-loglevel")
        .arg("warning")
        .output();

    match output {
        Ok(_) => {}
        Err(e) => eprintln!("Error executing command: {}", e),
    }

    // 合并文件后，删除.ogg和.m4s
    match fs::remove_file(v_title) {
        Ok(_) => {}
        Err(e) => eprintln!("删除文件时出错: {}", e),
    }
    match fs::remove_file(a_title) {
        Ok(_) => {}
        Err(e) => eprintln!("删除文件时出错: {}", e),
    }
}

// 特殊ffmpeg合并（无音频视频）
fn ffmpeg_none_a_url(v_title: String, video_save_path: String) {
    // 通过 FFmpeg 合并音视频文件为mp4文件
    let output = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(&v_title)
        .arg("-c")
        .arg("copy")
        .arg("-shortest")
        .arg(&video_save_path) // 合并输出目录和文件名
        .arg("-loglevel")
        .arg("warning")
        .output();

    match output {
        Ok(_) => {}
        Err(e) => eprintln!("Error executing command: {}", e),
    }

    // 合并文件后，删除.m4s
    match fs::remove_file(v_title) {
        Ok(_) => {}
        Err(e) => eprintln!("删除文件时出错: {}", e),
    }
}

// 更改视频文件的修改时间，改为up主上传文件的时间
fn update_file_change_time(pubdate: u64, path: String) {
    let duration = Duration::from_secs(pubdate);
    // 使用 UNIX_EPOCH 和 duration 创建 SystemTime
    let system_time = SystemTime::UNIX_EPOCH + duration;
    set_mtime(path.clone(), SystemTimeSpec::from(system_time)).expect("TODO: panic message");
}
