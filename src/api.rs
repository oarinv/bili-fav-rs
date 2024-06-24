use crate::read_cfg::read_config;
use reqwest::header::COOKIE;
use reqwest::header::USER_AGENT;
use reqwest::{header, Response};

// get api
pub async fn get_api(api: String) -> Result<Response, reqwest::Error> {
    let base_url = "https://api.bilibili.com";
    let user_agent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36";
    let url = format!("{}{}", base_url, api);

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

    Ok(res)
}
