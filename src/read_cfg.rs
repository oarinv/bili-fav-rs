use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    config: SomeConfig,
}

#[derive(Debug, Deserialize)]
struct SomeConfig {
    sessdata: String, // 从cookies中获取
    fav_mid: String,  // 收藏夹fav_mid
    fav_mode: String, // 收藏夹获取模式，first为获取第一页，all为获取所有页
    dir_path: String, // 保存视频的地址
}

// init config 初始化配置
fn init_config() -> Config {
    let toml_str = fs::read_to_string("config.toml").expect("not found config.toml");
    let config: Config = toml::from_str(&toml_str).expect("read toml error");
    return config;
}

// 读取配置
pub fn read_config() -> (String, String, String) {
    let config = init_config();
    let sessdata = config.config.sessdata;
    let fav_mid = config.config.fav_mid;
    let fav_mode = config.config.fav_mode;

    return (sessdata, fav_mid, fav_mode);
}

// 读取保存文件路径
pub fn read_dir_path() -> String {
    let config = init_config();
    let dir_path = config.config.dir_path;
    let save_path = format!("{}/", dir_path);

    return save_path;
}
