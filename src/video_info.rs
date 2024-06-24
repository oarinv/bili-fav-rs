use crate::api::get_api;
use crate::read_cfg::read_dir_path;
use crate::video_url::video_url;
use serde_json::Value;
use std::fs;

pub async fn video_info(bv_id: String) -> Result<bool, Box<dyn std::error::Error>> {
    let api = format!("{}{}", "/x/web-interface/view?bvid=", bv_id);

    if let Ok(res) = get_api(api).await {
        if res.status().is_success() {
            let body = res.text().await?;
            let json_value: Value = serde_json::from_str(&body)?;

            // 添加对于失效视频的检查，如果视频在哔哩哔哩不存在，则通过返回布尔值，判断，跳出此次循环
            let code = match json_value["code"].as_u64() {
                Some(value) => value.to_string(),
                None => "200".to_string(),
            };
            if code == "62002" {
                return Ok(true);
            }

            // 视频cid
            let cid = json_value["data"]["cid"]
                .as_u64()
                .ok_or("CID not found in the response")?
                .to_string();
            // 视频标题
            let title = json_value["data"]["title"]
                .as_str()
                .ok_or("Title not found in the response")?
                .to_string();
            // 投稿时间
            let pubdate = json_value["data"]["pubdate"]
                .as_u64()
                .ok_or("pubdate not found in the response")?;

            // 视频保存标题，包含bvid信息
            let save_title = format!("{}[{}]", to_save_filename(&title), bv_id.clone());

            let dir_path = read_dir_path();
            let v_path = format!("{}{}.m4s", &dir_path, &save_title);
            let a_path = format!("{}{}.ogg", &dir_path, &save_title);
            let video_save_path = format!("{}{}.mp4", &dir_path, &save_title);

            if fs::metadata(&video_save_path).is_err() {
                wd_log::log_info_ln!("{}", &save_title);
                println!("{}", &save_title);
                if let Err(err) = video_url(
                    bv_id.clone(),
                    cid,
                    v_path.clone(),
                    a_path.clone(),
                    video_save_path.clone(),
                    pubdate,
                )
                .await
                {
                    return Err(err.into());
                }
            }
            Ok(false)
        } else {
            wd_log::log_warn_ln!("保存错误 {:?}", res.status());
            Ok(false)
        }
    } else {
        Ok(false)
        // Handle the error or log it
    }
}

// 规范化文件名，防止保存出错
fn to_save_filename(input: &str) -> String {
    let replaced = input
        .replace("/", " ")
        .replace("\\", " ")
        .replace("<", " ")
        .replace(">", " ")
        .replace(":", " ")
        .replace("\"", " ")
        .replace("|", " ")
        .replace("?", " ")
        .replace("*", " ");

    replaced
}
