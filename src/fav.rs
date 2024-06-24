use crate::api::get_api;
use crate::video_info::video_info;
use serde_json::Value;

// 获取第一页的bv_id
// 获取收藏夹中的bv_id
pub async fn get_fav_videos(
    fav_id: String,
    all_pages: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // 如果all_pages为true，则下载所有的收藏夹的视频
    // 否则下载第一页的20个视频
    let api = if all_pages {
        format!("/x/v3/fav/resource/ids?media_id={}", fav_id)
    } else {
        format!("/x/v3/fav/resource/list?media_id={}&ps=20&pn=1", fav_id)
    };

    let res = get_api(api).await?;

    if !res.status().is_success() {
        return Ok(());
    }

    let body = res.text().await?;
    let json_value: Value = serde_json::from_str(&body)?;

    if let Some(medias) = if all_pages {
        json_value["data"].as_array()
    } else {
        json_value["data"]["medias"].as_array()
    } {
        for media in medias {
            if let Some(bv_id) = media["bv_id"].as_str() {
                let is_black = video_info(String::from(bv_id)).await?;

                // 通过video_info传来的布尔值，做判断，对于失效视频的检查，如果视频在哔哩哔哩不存在，则通过返回布尔值，判断，跳出此次循环
                if is_black == true {
                    break;
                }
            } else {
                wd_log::log_warn_ln!("bv_id not found in a media item");
            }
        }

        // 程序运行完毕提示
        println!("Done");
    } else {
        wd_log::log_warn_ln!("medias array not found");
    }

    Ok(())
}
