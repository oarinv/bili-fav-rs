use crate::api::get_api;
use crate::save_video::save_video;
use serde_json::Value;

// 获取视频链接和音频链接
pub(crate) async fn video_url(
    bv_id: String,           // 视频bvid必须
    cid: String,             // 视频cid获取视频下载链接必须
    v_path: String,          // 视频保存路径
    a_path: String,          // 音频保存路径
    video_save_path: String, // 视频保存路径
    pubdate: u64,            // 视频上传时间
) -> Result<(), Box<dyn std::error::Error>> {
    let api = format!("/x/player/wbi/playurl?cid={}&bvid={}&fnval=16", cid, bv_id);

    if let Ok(res) = get_api(api).await {
        if res.status().is_success() {
            let body = res.text().await?;
            let json_value: Value = serde_json::from_str(&body)?;

            if let (Some(video_url), Some(audio_url)) = (
                json_value["data"]["dash"]["video"][0]["baseUrl"].as_str(),
                json_value["data"]["dash"]["audio"][0]["baseUrl"].as_str(),
            ) {
                let v_url = video_url.to_string(); // 获取视频下载链接
                let a_url = audio_url.to_string(); // 获取音频下载链接

                save_video(v_url, a_url, v_path, a_path, video_save_path, pubdate).await?;
            }
        } else {
            wd_log::log_warn_ln!("网络错误 {:?}", res.status());
        }
    } else {
        // Handle the error or log it
    }

    Ok(())
}
