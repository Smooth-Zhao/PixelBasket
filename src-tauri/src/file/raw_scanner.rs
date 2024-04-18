use std::io::Cursor;
use std::path::Path;
use std::process::Command;

use base64::{Engine as _, engine::general_purpose};
use image::{DynamicImage, GenericImageView, ImageFormat, RgbImage};
use kmeans_colors::{Calculate, CentroidData, get_kmeans_hamerly, Sort};
use palette::{FromColor, IntoColor, Srgb};
use palette::cast::ComponentsAs;
use tokio::sync::mpsc::Sender;

use crate::file::metadata::Metadata;
use crate::file::scan::Scanner;
use crate::{debug, Result};
use crate::util::error::ErrorHandle;

pub struct RawScanner {}

impl RawScanner {
    pub fn wrap() -> Box<Self> {
        Box::new(RawScanner {})
    }
}

impl Scanner for RawScanner {
    fn is_support(&self, suffix: &str) -> bool {
        match suffix {
            "nef" => true,
            _ => false,
        }
    }

    fn scan(
        &self,
        path: &Path,
        tx: Sender<String>
    ) -> Result<()> {
        let mut metadata = Metadata::load(path);
        let path = path.to_path_buf();
        if self.is_support(metadata.file_suffix.as_str()) {
            tokio::spawn(async move {
                if metadata.analyze_metadata(&path).is_ok() {
                    if analyze_raw_metadata(&path, &mut metadata).is_ok() {
                        metadata.save_to_db().await;
                        tx.send(metadata.file_path).await.print_error();
                    };
                };
            });
        }
        Ok(())
    }
}

/// 解析图片元数据
fn analyze_raw_metadata(path: &Path, metadata: &mut Metadata) -> Result<()> {
    let image = rawloader::decode_file(path).expect("error loading image");

    metadata.image_width = image.width as u32;
    metadata.image_height = image.height as u32;

    if let Ok(base64) = thumbnail(path) {
        metadata.thumbnail = base64;
    }



    // let resize_image = thumbnail(&image, metadata.image_width, metadata.image_height);
    // if let Some(base64) = image_to_base64(&resize_image) {
    //     metadata.thumbnail = base64;
    // }
    // metadata.colors = kmeans(&resize_image);
    // metadata.shape = calculated_shape(metadata.image_width, metadata.image_height);
    Ok(())
}
/// 生成图片缩咯图
fn thumbnail(path: &Path) -> Result<String> {
    // 调用 bin/raw2base64.exe
    let output = Command::new("bin/raw2base64.exe")
        .args(&[path.to_str().unwrap()])
        .output()?;
    // 检查命令执行是否成功
    if !output.status.success() {
        return Err("Failed to execute ffmpeg command".into());
    }
    let buffer = output.stdout;
    // buffer转成字符串

    Ok(format!("data:image/jpg;base64,{}", String::from_utf8(buffer).unwrap()))
}
