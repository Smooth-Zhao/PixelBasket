use std::io::Cursor;
use std::path::Path;

use base64::{Engine as _, engine::general_purpose};
use chrono::offset::Utc;
use image::{DynamicImage, GenericImageView, ImageFormat, RgbImage};
use kmeans_colors::{Calculate, CentroidData, get_kmeans_hamerly, Sort};
use palette::{FromColor, IntoColor, Srgb};
use palette::cast::ComponentsAs;
use sqlx::query;
use tokio::sync::mpsc::Sender;

use crate::db::sqlite::Session;
use crate::file::metadata::{ImageMetadata, Metadata};
use crate::file::scan::Scanner;
use crate::Result;
use crate::util::error::ErrorHandle;
use crate::util::snowflake::id;

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

    fn scan(&self, path: &Path, tx: Sender<String>) -> Result<()> {
        let mut metadata = Metadata::load(path);
        let path = path.to_path_buf();
        if self.is_support(metadata.file_suffix.as_str()) {
            tokio::spawn(async move {
                if metadata.analyze_metadata(&path).is_ok() {
                    let mut image_metadata = ImageMetadata::new(metadata);
                    if analyze_image_metadata(&path, &mut image_metadata).is_ok() {
                        save_to_db(&image_metadata).await;
                        tx.send(image_metadata.metadata.file_path)
                            .await
                            .print_error();
                    };
                };
            });
        }
        Ok(())
    }
}

/// 解析图片元数据
fn analyze_image_metadata(path: &Path, image_metadata: &mut ImageMetadata) -> Result<()> {
    let image = image::open(path)?;
    let dimensions = image.dimensions();
    image_metadata.image_width = dimensions.0;
    image_metadata.image_height = dimensions.1;
    let resize_image = thumbnail(
        &image,
        image_metadata.image_width,
        image_metadata.image_height,
    );
    if let Some(base64) = image_to_base64(&resize_image) {
        image_metadata.thumbnail = base64;
    }
    image_metadata.colors = kmeans(&resize_image);
    image_metadata.shape =
        calculated_shape(image_metadata.image_width, image_metadata.image_height);
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

async fn save_to_db(image_metadata: &ImageMetadata) {
    let mut session = Session::new("./db/main.db");
    session.connect().await;
    if let Ok(pool) = &session.get_pool() {
        if let Ok(result) = session
            .count(
                format!(
                    "SELECT COUNT(*) AS count FROM metadata WHERE sha1 = '{}'",
                    &image_metadata.metadata.sha1
                )
                .as_str(),
            )
            .await
        {
            if result.count == 0 {
                let _ = query("INSERT INTO metadata (id, file_name, file_path, file_size, file_suffix, added, created, modified, image_width, image_height, thumbnail, tags, exegesis, score, colors, shape, duration, is_del, sha1) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                        .bind(id::<i64>())
                        .bind(&image_metadata.metadata.file_name)
                        .bind(&image_metadata.metadata.file_path)
                        .bind(image_metadata.metadata.file_size as i64)
                        .bind(&image_metadata.metadata.file_suffix)
                        .bind(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string())
                        .bind(&image_metadata.metadata.created)
                        .bind(&image_metadata.metadata.modified)
                        .bind(image_metadata.image_width)
                        .bind(image_metadata.image_height)
                        .bind(&image_metadata.thumbnail)
                        .bind(&image_metadata.metadata.tags)
                        .bind(&image_metadata.metadata.exegesis)
                        .bind(image_metadata.metadata.score)
                        .bind(&image_metadata.colors)
                        .bind(&image_metadata.shape)
                        .bind(0)
                        .bind(0)
                        .bind(&image_metadata.metadata.sha1)
                        .execute(pool)
                        .await;
            }
        }
    }
}
