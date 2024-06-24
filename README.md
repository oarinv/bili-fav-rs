# Bilibili Favorites Downloader

## 简介

这是一个用 Rust 语言编写的程序，旨在下载 Bilibili 收藏夹中的视频。

## 使用前提

在使用该程序之前，请确保你已经拥有以下信息：

1. Bilibili 用户的 Cookie（sessdata）
2. 目标收藏夹的 MID（fav_mid）
3. 获取收藏夹的模式，（fav_mode) first为第一页，all为所有页面
4. 保存视频的路径（dir_path）

这些信息将在配置文件 `config.toml` 中进行设置。

## 配置文件示例

```toml
[config]
sessdata = 'YOUR_BILIBILI_COOKIE'
fav_mid  = 'TARGET_FAVORITES_MID'
fav_mode = 'fisrt&all'
dir_path = '/path/to/save/videos'
```

请将 `YOUR_BILIBILI_COOKIE` 替换为你的 bilibili 用户 Cookie，`TARGET_FAVORITES_MID` 替换为目标收藏夹的 MID。


## 如何运行程序

1. 在终端中导航到程序源代码所在的目录。
2. 执行以下命令：

```bash
cargo run
```

程序将读取 `config.toml` 文件中的配置信息，并开始下载视频到指定路径。

## 注意事项

- 请确保你拥有下载目标收藏夹中视频的权限。

## 许可证

本程序基于 [MIT 许可证](LICENSE) 开源。LICENSE)。详细信息请参阅许可证文件。
