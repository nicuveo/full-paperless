use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::utils;

#[readonly::make]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationConfiguration {
    #[readonly]
    pub id: i32,
    pub user_args: serde_json::Value,
    pub barcode_tag_mapping: serde_json::Value,
    #[serde(deserialize_with = "utils::blank_enum")]
    pub output_type: Option<super::OutputType>,
    pub pages: Option<i64>,
    pub language: Option<String>,
    #[serde(deserialize_with = "utils::blank_enum")]
    pub mode: Option<super::OcrMode>,
    #[serde(deserialize_with = "utils::blank_enum")]
    pub skip_archive_file: Option<super::SkipArchiveFile>,
    pub image_dpi: Option<i64>,
    #[serde(deserialize_with = "utils::blank_enum")]
    pub unpaper_clean: Option<super::UnpaperClean>,
    pub deskew: Option<bool>,
    pub rotate_pages: Option<bool>,
    pub rotate_pages_threshold: Option<f64>,
    pub max_image_pixels: Option<f64>,
    #[serde(deserialize_with = "utils::blank_enum")]
    pub color_conversion_strategy: Option<super::ColorConversionStrategy>,
    pub app_title: Option<String>,
    pub app_logo: Option<String>,
    pub barcodes_enabled: Option<bool>,
    pub barcode_enable_tiff_support: Option<bool>,
    pub barcode_string: Option<String>,
    pub barcode_retain_split_pages: Option<bool>,
    pub barcode_enable_asn: Option<bool>,
    pub barcode_asn_prefix: Option<String>,
    pub barcode_upscale: Option<f64>,
    pub barcode_dpi: Option<i64>,
    pub barcode_max_pages: Option<i64>,
    pub barcode_enable_tag: Option<bool>,
}
