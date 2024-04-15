use std::io::Cursor;
use std::path::Path;

use base64::{Engine as _, engine::general_purpose};
use image::{DynamicImage, GenericImageView, ImageFormat, RgbImage};
use kmeans_colors::{Calculate, CentroidData, get_kmeans_hamerly, Sort};
use palette::{FromColor, IntoColor, Srgb};
use palette::cast::ComponentsAs;
use tokio::sync::mpsc::Sender;

use crate::file::metadata::Metadata;
use crate::file::scan::Scanner;
use crate::Result;
use crate::util::error::ErrorHandle;

pub struct ImageScanner {}

impl ImageScanner {
    pub fn wrap() -> Box<Self> {
        Box::new(ImageScanner {})
    }
}

impl Scanner for ImageScanner {
    fn is_support(&self, suffix: &str) -> bool {
        match suffix {
            "avif" | "bmp" | "dds" | "farbfeld" | "gif" | "hdr" | "ico" | "jpg" | "jpeg"
            | "exr" | "png" | "pnm" | "qoi" | "tga" | "tiff" | "webp" => true,
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
                    if analyze_image_metadata(&path, &mut metadata).is_ok() {
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
fn analyze_image_metadata(path: &Path, metadata: &mut Metadata) -> Result<()> {
    let image = image::open(path)?;
    let dimensions = image.dimensions();
    metadata.image_width = dimensions.0;
    metadata.image_height = dimensions.1;
    let resize_image = thumbnail(&image, metadata.image_width, metadata.image_height);
    if let Some(base64) = image_to_base64(&resize_image) {
        metadata.thumbnail = base64;
    }
    metadata.colors = kmeans(&resize_image);
    metadata.shape = calculated_shape(metadata.image_width, metadata.image_height);
    Ok(())
}

/// 生成图片缩咯图
fn thumbnail(image: &DynamicImage, w: u32, h: u32) -> RgbImage {
    let w1 = 200;
    let h1 = (200f32 / w as f32 * h as f32) as u32;
    image.thumbnail(w1, h1).to_rgb8()
}

/// 生成base64图片
fn image_to_base64(image: &RgbImage) -> Option<String> {
    let mut buffer = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Jpeg)
        .print_error();
    if !buffer.is_empty() {
        let base64 = general_purpose::STANDARD.encode(&buffer);
        Some(format!("data:image/jpg;base64,{}", base64))
    } else {
        None
    }
}

/// 提取主题色
fn kmeans(image: &RgbImage) -> String {
    let img_vec: &[Srgb<u8>] = image.as_raw().components_as();

    let mut rgb_pixels: Vec<Srgb<f32>> = Vec::new();
    rgb_pixels.extend(
        img_vec
            .iter()
            .map(|x| Srgb::<f32>::from_color(x.into_format())),
    );

    let result = get_kmeans_hamerly(8, 1, 0.0025, false, &rgb_pixels, 0);
    let result = Srgb::sort_indexed_colors(&result.centroids, &result.indices);
    to_colors(&result)
}

fn to_colors<C: Calculate + Copy + IntoColor<Srgb>>(colors: &[CentroidData<C>]) -> String {
    colors
        .iter()
        .map(|x| format!("#{:x}", x.centroid.into_color().into_format::<u8>()))
        .collect::<Vec<String>>()
        .join(",")
}

fn calculated_shape(w: u32, h: u32) -> String {
    let divisor = greatest_common_divisor(w, h);
    format!("{}:{}", w / divisor, h / divisor)
}

fn greatest_common_divisor(a: u32, b: u32) -> u32 {
    if a == 0 {
        b
    } else {
        greatest_common_divisor(b % a, a)
    }
}
