use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Create {
    pub user_args: serde_json::Value,
    pub barcode_tag_mapping: serde_json::Value,
    pub output_type: Option<model::OutputType>,
    pub pages: Option<i64>,
    pub language: Option<String>,
    pub mode: Option<model::OcrMode>,
    pub skip_archive_file: Option<model::SkipArchiveFile>,
    pub image_dpi: Option<i64>,
    pub unpaper_clean: Option<model::UnpaperClean>,
    pub deskew: Option<bool>,
    pub rotate_pages: Option<bool>,
    pub rotate_pages_threshold: Option<f64>,
    pub max_image_pixels: Option<f64>,
    pub color_conversion_strategy: Option<model::ColorConversionStrategy>,
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

#[must_use]
pub fn create() -> Create {
    Create::new()
}

impl From<&model::ApplicationConfiguration> for Create {
    fn from(item: &model::ApplicationConfiguration) -> Self {
        Self {
            user_args: item.user_args.clone(),
            barcode_tag_mapping: item.barcode_tag_mapping.clone(),
            output_type: item.output_type,
            pages: item.pages,
            language: item.language.clone(),
            mode: item.mode,
            skip_archive_file: item.skip_archive_file,
            image_dpi: item.image_dpi,
            unpaper_clean: item.unpaper_clean,
            deskew: item.deskew,
            rotate_pages: item.rotate_pages,
            rotate_pages_threshold: item.rotate_pages_threshold,
            max_image_pixels: item.max_image_pixels,
            color_conversion_strategy: item.color_conversion_strategy,
            app_title: item.app_title.clone(),
            app_logo: item.app_logo.clone(),
            barcodes_enabled: item.barcodes_enabled,
            barcode_enable_tiff_support: item.barcode_enable_tiff_support,
            barcode_string: item.barcode_string.clone(),
            barcode_retain_split_pages: item.barcode_retain_split_pages,
            barcode_enable_asn: item.barcode_enable_asn,
            barcode_asn_prefix: item.barcode_asn_prefix.clone(),
            barcode_upscale: item.barcode_upscale,
            barcode_dpi: item.barcode_dpi,
            barcode_max_pages: item.barcode_max_pages,
            barcode_enable_tag: item.barcode_enable_tag,
        }
    }
}

impl Create {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn user_args(mut self, value: serde_json::Value) -> Self {
        self.user_args = value;
        self
    }

    #[must_use]
    pub fn barcode_tag_mapping(mut self, value: serde_json::Value) -> Self {
        self.barcode_tag_mapping = value;
        self
    }

    #[must_use]
    pub fn output_type(mut self, value: model::OutputType) -> Self {
        self.output_type = Some(value);
        self
    }

    #[must_use]
    pub fn pages(mut self, value: i64) -> Self {
        self.pages = Some(value);
        self
    }

    #[must_use]
    pub fn language(mut self, value: String) -> Self {
        self.language = Some(value);
        self
    }

    #[must_use]
    pub fn mode(mut self, value: model::OcrMode) -> Self {
        self.mode = Some(value);
        self
    }

    #[must_use]
    pub fn skip_archive_file(mut self, value: model::SkipArchiveFile) -> Self {
        self.skip_archive_file = Some(value);
        self
    }

    #[must_use]
    pub fn image_dpi(mut self, value: i64) -> Self {
        self.image_dpi = Some(value);
        self
    }

    #[must_use]
    pub fn unpaper_clean(mut self, value: model::UnpaperClean) -> Self {
        self.unpaper_clean = Some(value);
        self
    }

    #[must_use]
    pub fn deskew(mut self, value: bool) -> Self {
        self.deskew = Some(value);
        self
    }

    #[must_use]
    pub fn rotate_pages(mut self, value: bool) -> Self {
        self.rotate_pages = Some(value);
        self
    }

    #[must_use]
    pub fn rotate_pages_threshold(mut self, value: f64) -> Self {
        self.rotate_pages_threshold = Some(value);
        self
    }

    #[must_use]
    pub fn max_image_pixels(mut self, value: f64) -> Self {
        self.max_image_pixels = Some(value);
        self
    }

    #[must_use]
    pub fn color_conversion_strategy(mut self, value: model::ColorConversionStrategy) -> Self {
        self.color_conversion_strategy = Some(value);
        self
    }

    #[must_use]
    pub fn app_title(mut self, value: String) -> Self {
        self.app_title = Some(value);
        self
    }

    #[must_use]
    pub fn app_logo(mut self, value: String) -> Self {
        self.app_logo = Some(value);
        self
    }

    #[must_use]
    pub fn barcodes_enabled(mut self, value: bool) -> Self {
        self.barcodes_enabled = Some(value);
        self
    }

    #[must_use]
    pub fn barcode_enable_tiff_support(mut self, value: bool) -> Self {
        self.barcode_enable_tiff_support = Some(value);
        self
    }

    #[must_use]
    pub fn barcode_string(mut self, value: String) -> Self {
        self.barcode_string = Some(value);
        self
    }

    #[must_use]
    pub fn barcode_retain_split_pages(mut self, value: bool) -> Self {
        self.barcode_retain_split_pages = Some(value);
        self
    }

    #[must_use]
    pub fn barcode_enable_asn(mut self, value: bool) -> Self {
        self.barcode_enable_asn = Some(value);
        self
    }

    #[must_use]
    pub fn barcode_asn_prefix(mut self, value: String) -> Self {
        self.barcode_asn_prefix = Some(value);
        self
    }

    #[must_use]
    pub fn barcode_upscale(mut self, value: f64) -> Self {
        self.barcode_upscale = Some(value);
        self
    }

    #[must_use]
    pub fn barcode_dpi(mut self, value: i64) -> Self {
        self.barcode_dpi = Some(value);
        self
    }

    #[must_use]
    pub fn barcode_max_pages(mut self, value: i64) -> Self {
        self.barcode_max_pages = Some(value);
        self
    }

    #[must_use]
    pub fn barcode_enable_tag(mut self, value: bool) -> Self {
        self.barcode_enable_tag = Some(value);
        self
    }
}
