use std::path::Path;
use std::process::Command;

use base64::Engine;
use base64::engine::general_purpose;

use crate::db::entity::metadata::Metadata;
use crate::db::entity::task::{Task, TaskStatus};
use crate::file::scan::{Context, Scanner};
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

    fn scan(&self, task: &Task, context: &Context) -> TaskStatus {
        let mut status = TaskStatus::new(task.id);
        if self.is_support(task.file_suffix.as_str()) {
            let path = task.file_path.clone();
            let runtime = context.runtime.handle().clone();
            status.handle(runtime.clone().spawn_blocking(move || {
                let path = Path::new(path.as_str());
                let mut metadata = Metadata::load(path);
                if metadata.analyze_metadata(path).is_ok() {
                    if analyze_video_metadata(path, &mut metadata).is_ok() {
                        // 使用阻塞线程防止数据丢失！
                        runtime.block_on(async move {
                            metadata.save_to_db().await;
                        });
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
    metadata.duration = duration(path)?;
    metadata.thumbnail = thumbnail(path)?;

    Ok(())
}


fn duration(path: &Path) -> Result<i64>{
    // 定义 FFmpeg 命令
    let output = Command::new("ffprobe")
        .args(&[
            "-v", "error",             // 设置输出日志级别为 error，以避免输出额外信息
            "-show_entries", "format=duration",  // 指定要显示的信息为视频时长
            "-of", "default=noprint_wrappers=1:nokey=1",  // 设置输出格式为纯文本
            path.to_str().unwrap()               // 输入视频文件
        ])
        .output()?;         // 执行 FFmpeg 命令并获取输出

    let duration_str = String::from_utf8_lossy(&output.stdout);
    let duration: f64 = duration_str.trim().parse().unwrap();
    Ok((duration * 1000f64) as i64)
}
/// 生成缩咯图
fn thumbnail(path: &Path) -> Result<String> {
    // 定义 ffmpeg 命令
    let output = Command::new("ffmpeg")
        .args(&[
            "-i",
            path.to_str().unwrap(),
            "-vf",
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

    // 将输出转换为 Base64 编码的字符串
    let base64 = general_purpose::STANDARD.encode(&buffer);

    Ok(format!("data:image/jpg;base64,{}", base64))
}
