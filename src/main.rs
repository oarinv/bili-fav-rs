mod api; //通用api调用接口
mod fav; //获取收藏夹视频
mod log; //初始化日志
mod read_cfg; //从config.toml读取配置
mod save_video; //保存视频
mod video_info; //获取视频相关信息
mod video_url; //获取视频下载链接

use crate::api::get_api;
use crate::fav::get_fav_videos;
use crate::log::init_logging;
use crate::read_cfg::read_config;
use serde_json::Value;
use std::process::Command;

#[tokio::main]
async fn main() {
    // 初始化日志系统
    let _ = init_logging();

    // 读取配置
    let (_s, fav_id, fav_mode) = read_config();

    // 检查ffmpeg是否存在
    let ffmpeg_is_true = ffmpeg_exists();
    if ffmpeg_is_true == false {
        wd_log::log_warn_ln!("FFmpeg is not found");
        println!("FFmpeg is not found");
        return;
    }

    // 检查是否登陆（检查sessdata有效）
    if let Err(err) = check_is_login().await {
        wd_log::log_warn_ln!("Login check failed: {}", err);
        return;
    }

    // 读取配置中的模式
    let result = match fav_mode.as_str() {
        "first" => get_fav_videos(fav_id.clone(), false).await,
        "all" => get_fav_videos(fav_id.clone(), true).await,
        _ => {
            wd_log::log_warn_ln!("'{}' is an invalid config", fav_mode);
            return;
        }
    };

    if let Err(err) = result {
        wd_log::log_warn_ln!("Failed to retrieve favorite videos: {}", err);
    }
}

// 检查是否登陆，是则打印 hello 用户名，否则退出程序
async fn check_is_login() -> Result<(), Box<dyn std::error::Error>> {
    let api = "/x/space/myinfo".to_string();

    let res = get_api(api).await?;

    if res.status().is_success() {
        let body = res.text().await?;
        let json_value: Value = serde_json::from_str(&body)?;

        if let Some(code) = json_value["data"]["name"].as_str() {
            if code == "None" {
                return Err("未登陆".into());
            } else {
                println!("hello {}", code);
            }
        } else {
            wd_log::log_warn_ln!("Name not found in the response");
            // fix login err
            return Err("未登陆".into());
        }
    } else {
        wd_log::log_warn_ln!("Internet error");
    }

    Ok(())
}

// 检查ffmpeg是否存在
fn ffmpeg_exists() -> bool {
    match Command::new("ffmpeg").arg("-version").output() {
        Ok(_) => true,
        Err(_) => false,
    }
}
