use std::io::Cursor;
use std::path::Path;

use base64::{engine::general_purpose, Engine as _};
use chrono::offset::Utc;
use chrono::DateTime;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageFormat};

use crate::file::scan::Scanner;

fn resize_to_base64(image: DynamicImage, w: u32, h: u32) -> Option<String> {
    let w1 = 200;
    let h1 = (200f32 / w as f32 * h as f32) as u32;
    let small = image.resize(w1, h1, FilterType::Triangle);
    let mut buffer = Vec::new();
    if small
        .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Jpeg)
        .is_ok()
    {
        let base64 = general_purpose::STANDARD.encode(&buffer);
        return Some(format!("data:image/jpg;base64,{}", base64));
    }
    None
}

pub struct ImageMetadata {
    file_path: String,
    file_name: String,
    file_size: u64,
    file_suffix: String,
    created: String,
    modified: String,
    image_width: u32,
    image_height: u32,
}

pub struct ImageScanner {}

impl ImageScanner {
    pub fn wrap() -> Box<Self> {
        Box::new(ImageScanner {})
    }
}

impl Scanner for ImageScanner {
    fn scan(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            if let Some(file_suffix) = extension.to_str() {
                match file_suffix.to_lowercase().as_str() {
                    "avif" | "bmp" | "dds" | "farbfeld" | "gif" | "hdr" | "ico" | "jpg"
                    | "jpeg" | "exr" | "png" | "pnm" | "qoi" | "tga" | "tiff" | "webp" => {
                        println!("==> {}", path.display());
                        if let Some(file_name) = path.file_name() {
                            println!("文件名：{}", file_name.to_string_lossy());
                        }
                        let mut file_size: u64 = 0;
                        if let Ok(metadata) = path.metadata() {
                            file_size = metadata.len();
                            println!("文件大小：{}", file_size);
                            println!("格式：{:?}", file_suffix);
                            if let Ok(time) = metadata.created() {
                                let datetime: DateTime<Utc> = time.into();
                                println!("创建日期：{}", datetime.format("%Y-%m-%d %H:%M:%S"));
                            }
                            if let Ok(time) = metadata.modified() {
                                let datetime: DateTime<Utc> = time.into();
                                println!("修改日期：{}", datetime.format("%Y-%m-%d %H:%M:%S"));
                            }
                        }
                        if let Ok(image) = image::open(path) {
                            let dimensions = image.dimensions();
                            let w = dimensions.0;
                            let h = dimensions.1;
                            println!("尺寸：{} * {}", w, h);
                            if let Some(base64) = resize_to_base64(image, w, h) {
                                println!("缩咯图：{}", base64)
                            }
                        }
                        return true;
                    }
                    _ => {}
                }
            }
        }
        false
    }
}
