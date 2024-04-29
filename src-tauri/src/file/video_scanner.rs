use base64::engine::general_purpose;
use base64::Engine;
use std::path::Path;
use std::process::Command;

use crate::db::entity::metadata::Metadata;
use crate::db::entity::task::{Task, TaskStatus};
use crate::file::scan::Scanner;
use crate::Result;

pub struct VideoScanner {}

impl VideoScanner {
    pub fn wrap() -> Box<Self> {
        Box::new(VideoScanner {})
    }
}

impl Scanner for VideoScanner {
    fn is_support(&self, suffix: &str) -> bool {
        match suffix {
            "mp4" | "webm" | "ogg" => true,
            _ => false,
        }
    }

    fn scan(&self, task: &Task) -> TaskStatus {
        let mut status = TaskStatus::new(task.id);
        if self.is_support(task.file_suffix.as_str()) {
            let path = task.file_path.clone();
            status.handle(tokio::spawn(async move {
                let path = Path::new(path.as_str());
                let mut metadata = Metadata::load(path);
                if metadata.analyze_metadata(path).is_ok() {
                    if analyze_video_metadata(path, &mut metadata).is_ok() {
                        metadata.save_to_db().await;
                        return true;
                    };
                };
                false
            }));
        }
        status
    }
}
fn analyze_video_metadata(path: &Path, metadata: &mut Metadata) -> Result<()> {
    metadata.thumbnail = thumbnail(path)?;

    Ok(())
}

/// 生成缩咯图
fn thumbnail(path: &Path) -> Result<String> {
    // 定义 ffmpeg 命令
    let output = Command::new("ffmpeg")
        .args(&[
            "-i",
            path.to_str().unwrap(),
            "\
        -vf",
            "thumbnail,scale=320:-1",
            "-frames:v",
            "1",
            "-f",
            "image2pipe",
            "-",
        ])
        .output()?;
    // 检查命令执行是否成功
    if !output.status.success() {
        return Err("Failed to execute ffmpeg command".into());
    }
    let buffer = output.stdout;

    // 打开视频文件
    //     let input = input(path)?;
    //     // 查找视频流
    //     let video_stream = input.streams().best(Type::Video).ok_or(ffmpeg_next::Error::StreamNotFound)?;
    //
    //     // 创建过滤器图
    //     let mut filter_graph = Graph::new();
    //
    //     // 添加输入流
    //     let mut input_stream = filter_graph.add_input_from_stream(&video_stream)?;
    //
    //     // 添加thumbnail过滤器
    //     let mut filter = filter_graph.add("thumbnail", &[])?;
    //
    //     // 设置参数
    //     filter.set("frames", "1")?; // 设置提取的帧数量
    //     filter.set("scale", "320:-1")?; // 设置从第一帧开始提取
    //
    //     // 链接过滤器和输出流
    //     let mut output_stream = filter.outputs().get_mut(0).ok_or(ffmpeg_next::Error::StreamNotFound)?;
    //     output_stream.set_format(ffmpeg_next::format::Pixel::RGB24)?;
    //
    //     // 配置过滤器图
    //     filter_graph.configure()?;
    //
    //     // 执行过滤器图
    //     filter_graph.run()?;
    //
    //     // 保存输出
    //     let mut buffer = output_stream.pull_packet()?;

    // 将输出转换为 Base64 编码的字符串
    let base64 = general_purpose::STANDARD.encode(&buffer);

    Ok(format!("data:image/jpg;base64,{}", base64))
}
// fn duration(path: &Path) -> Result<f64> {
//
//     // 定义 ffprobe 命令
//     let output = Command::new("ffprobe")
//         .args(&["-v", "error", "-show_entries", "format=duration", "-of", "default=noprint_wrappers=1:nokey=1", path.to_str().unwrap()])
//         .output()?;
//
//     // 检查命令执行是否成功
//     if !output.status.success() {
//         return Err("Failed to execute ffprobe command".into());
//     }
//
//     // 将输出转换为字符串并解析为时长
//     let duration_str = str::?;
//     let duration: f64 = duration_str.trim().parse()?;
//
//     println!("Video duration: {} seconds", duration);
//
//     Ok(duration)
// }
