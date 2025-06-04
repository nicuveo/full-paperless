use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::utils;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct List {
    pub full_perms: bool,
    #[serde(rename = "added__date__gt")]
    pub added_date_gt: Option<String>,
    #[serde(rename = "added__date__gte")]
    pub added_date_gte: Option<String>,
    #[serde(rename = "added__date__lt")]
    pub added_date_lt: Option<String>,
    #[serde(rename = "added__date__lte")]
    pub added_date_lte: Option<String>,
    #[serde(rename = "added__day")]
    pub added_day: Option<f64>,
    #[serde(rename = "added__gt")]
    pub added_gt: Option<String>,
    #[serde(rename = "added__gte")]
    pub added_gte: Option<String>,
    #[serde(rename = "added__lt")]
    pub added_lt: Option<String>,
    #[serde(rename = "added__lte")]
    pub added_lte: Option<String>,
    #[serde(rename = "added__month")]
    pub added_month: Option<f64>,
    #[serde(rename = "added__year")]
    pub added_year: Option<f64>,
    #[serde(rename = "archive_serial_number")]
    pub archive_serial_number: Option<i32>,
    #[serde(rename = "archive_serial_number__gt")]
    pub archive_serial_number_gt: Option<i32>,
    #[serde(rename = "archive_serial_number__gte")]
    pub archive_serial_number_gte: Option<i32>,
    #[serde(rename = "archive_serial_number__isnull")]
    pub archive_serial_number_isnull: Option<bool>,
    #[serde(rename = "archive_serial_number__lt")]
    pub archive_serial_number_lt: Option<i32>,
    #[serde(rename = "archive_serial_number__lte")]
    pub archive_serial_number_lte: Option<i32>,
    #[serde(rename = "checksum__icontains")]
    pub checksum_icontains: Option<String>,
    #[serde(rename = "checksum__iendswith")]
    pub checksum_iendswith: Option<String>,
    #[serde(rename = "checksum__iexact")]
    pub checksum_iexact: Option<String>,
    #[serde(rename = "checksum__istartswith")]
    pub checksum_istartswith: Option<String>,
    #[serde(rename = "content__icontains")]
    pub content_icontains: Option<String>,
    #[serde(rename = "content__iendswith")]
    pub content_iendswith: Option<String>,
    #[serde(rename = "content__iexact")]
    pub content_iexact: Option<String>,
    #[serde(rename = "content__istartswith")]
    pub content_istartswith: Option<String>,
    #[serde(rename = "correspondent__id")]
    pub correspondent_id: Option<i32>,
    #[serde(rename = "correspondent__id__in", with = "utils::comma_list")]
    pub correspondent_id_in: Option<Vec<i32>>,
    #[serde(rename = "correspondent__id__none")]
    pub correspondent_id_none: Option<i32>,
    #[serde(rename = "correspondent__isnull")]
    pub correspondent_isnull: Option<bool>,
    #[serde(rename = "correspondent__name__icontains")]
    pub correspondent_name_icontains: Option<String>,
    #[serde(rename = "correspondent__name__iendswith")]
    pub correspondent_name_iendswith: Option<String>,
    #[serde(rename = "correspondent__name__iexact")]
    pub correspondent_name_iexact: Option<String>,
    #[serde(rename = "correspondent__name__istartswith")]
    pub correspondent_name_istartswith: Option<String>,
    #[serde(rename = "created__date__gt")]
    pub created_date_gt: Option<String>,
    #[serde(rename = "created__date__gte")]
    pub created_date_gte: Option<String>,
    #[serde(rename = "created__date__lt")]
    pub created_date_lt: Option<String>,
    #[serde(rename = "created__date__lte")]
    pub created_date_lte: Option<String>,
    #[serde(rename = "created__day")]
    pub created_day: Option<f64>,
    #[serde(rename = "created__gt")]
    pub created_gt: Option<String>,
    #[serde(rename = "created__gte")]
    pub created_gte: Option<String>,
    #[serde(rename = "created__lt")]
    pub created_lt: Option<String>,
    #[serde(rename = "created__lte")]
    pub created_lte: Option<String>,
    #[serde(rename = "created__month")]
    pub created_month: Option<f64>,
    #[serde(rename = "created__year")]
    pub created_year: Option<f64>,
    #[serde(rename = "custom_field_query")]
    pub custom_field_query: Option<String>,
    #[serde(rename = "custom_fields__icontains")]
    pub custom_fields_icontains: Option<String>,
    #[serde(rename = "custom_fields__id__all")]
    pub custom_fields_id_all: Option<i32>,
    #[serde(rename = "custom_fields__id__in")]
    pub custom_fields_id_in: Option<i32>,
    #[serde(rename = "custom_fields__id__none")]
    pub custom_fields_id_none: Option<i32>,
    #[serde(rename = "document_type__id")]
    pub document_type_id: Option<i32>,
    #[serde(rename = "document_type__id__in", with = "utils::comma_list")]
    pub document_type_id_in: Option<Vec<i32>>,
    #[serde(rename = "document_type__id__none")]
    pub document_type_id_none: Option<i32>,
    #[serde(rename = "document_type__isnull")]
    pub document_type_isnull: Option<bool>,
    #[serde(rename = "document_type__name__icontains")]
    pub document_type_name_icontains: Option<String>,
    #[serde(rename = "document_type__name__iendswith")]
    pub document_type_name_iendswith: Option<String>,
    #[serde(rename = "document_type__name__iexact")]
    pub document_type_name_iexact: Option<String>,
    #[serde(rename = "document_type__name__istartswith")]
    pub document_type_name_istartswith: Option<String>,
    #[serde(with = "utils::comma_list")]
    pub fields: Option<Vec<String>>,
    pub has_custom_fields: Option<bool>,
    pub id: Option<i32>,
    #[serde(rename = "id__in", with = "utils::comma_list")]
    pub id_in: Option<Vec<i32>>,
    #[serde(rename = "is_in_inbox")]
    pub is_in_inbox: Option<bool>,
    #[serde(rename = "is_tagged")]
    pub is_tagged: Option<bool>,
    #[serde(rename = "mime_type")]
    pub mime_type: Option<String>,
    #[serde(rename = "modified__date__gt")]
    pub modified_date_gt: Option<String>,
    #[serde(rename = "modified__date__gte")]
    pub modified_date_gte: Option<String>,
    #[serde(rename = "modified__date__lt")]
    pub modified_date_lt: Option<String>,
    #[serde(rename = "modified__date__lte")]
    pub modified_date_lte: Option<String>,
    #[serde(rename = "modified__day")]
    pub modified_day: Option<f64>,
    #[serde(rename = "modified__gt")]
    pub modified_gt: Option<String>,
    #[serde(rename = "modified__gte")]
    pub modified_gte: Option<String>,
    #[serde(rename = "modified__lt")]
    pub modified_lt: Option<String>,
    #[serde(rename = "modified__lte")]
    pub modified_lte: Option<String>,
    #[serde(rename = "modified__month")]
    pub modified_month: Option<f64>,
    #[serde(rename = "modified__year")]
    pub modified_year: Option<f64>,
    #[serde(rename = "ordering")]
    pub ordering: Option<String>,
    #[serde(rename = "original_filename__icontains")]
    pub original_filename_icontains: Option<String>,
    #[serde(rename = "original_filename__iendswith")]
    pub original_filename_iendswith: Option<String>,
    #[serde(rename = "original_filename__iexact")]
    pub original_filename_iexact: Option<String>,
    #[serde(rename = "original_filename__istartswith")]
    pub original_filename_istartswith: Option<String>,
    #[serde(rename = "owner__id")]
    pub owner_id: Option<i32>,
    #[serde(rename = "owner__id__in", with = "utils::comma_list")]
    pub owner_id_in: Option<Vec<i32>>,
    #[serde(rename = "owner__id__none")]
    pub owner_id_none: Option<i32>,
    #[serde(rename = "owner__isnull")]
    pub owner_isnull: Option<bool>,
    #[serde(rename = "page")]
    pub page: Option<i32>,
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    #[serde(rename = "search")]
    pub search: Option<String>,
    #[serde(rename = "shared_by__id")]
    pub shared_by_id: Option<bool>,
    #[serde(rename = "storage_path__id")]
    pub storage_path_id: Option<i32>,
    #[serde(rename = "storage_path__id__in", with = "utils::comma_list")]
    pub storage_path_id_in: Option<Vec<i32>>,
    #[serde(rename = "storage_path__id__none")]
    pub storage_path_id_none: Option<i32>,
    #[serde(rename = "storage_path__isnull")]
    pub storage_path_isnull: Option<bool>,
    #[serde(rename = "storage_path__name__icontains")]
    pub storage_path_name_icontains: Option<String>,
    #[serde(rename = "storage_path__name__iendswith")]
    pub storage_path_name_iendswith: Option<String>,
    #[serde(rename = "storage_path__name__iexact")]
    pub storage_path_name_iexact: Option<String>,
    #[serde(rename = "storage_path__name__istartswith")]
    pub storage_path_name_istartswith: Option<String>,
    #[serde(rename = "tags__id")]
    pub tags_id: Option<i32>,
    #[serde(rename = "tags__id__all")]
    pub tags_id_all: Option<i32>,
    #[serde(rename = "tags__id__in")]
    pub tags_id_in: Option<i32>,
    #[serde(rename = "tags__id__none")]
    pub tags_id_none: Option<i32>,
    #[serde(rename = "tags__name__icontains")]
    pub tags_name_icontains: Option<String>,
    #[serde(rename = "tags__name__iendswith")]
    pub tags_name_iendswith: Option<String>,
    #[serde(rename = "tags__name__iexact")]
    pub tags_name_iexact: Option<String>,
    #[serde(rename = "tags__name__istartswith")]
    pub tags_name_istartswith: Option<String>,
    #[serde(rename = "title__icontains")]
    pub title_icontains: Option<String>,
    #[serde(rename = "title__iendswith")]
    pub title_iendswith: Option<String>,
    #[serde(rename = "title__iexact")]
    pub title_iexact: Option<String>,
    #[serde(rename = "title__istartswith")]
    pub title_istartswith: Option<String>,
    #[serde(rename = "title_content")]
    pub title_content: Option<String>,
}

#[must_use]
pub fn list() -> List {
    List::new()
}

impl Default for List {
    #[allow(clippy::too_many_lines)]
    fn default() -> Self {
        Self {
            full_perms: true,
            added_date_gt: None,
            added_date_gte: None,
            added_date_lt: None,
            added_date_lte: None,
            added_day: None,
            added_gt: None,
            added_gte: None,
            added_lt: None,
            added_lte: None,
            added_month: None,
            added_year: None,
            archive_serial_number: None,
            archive_serial_number_gt: None,
            archive_serial_number_gte: None,
            archive_serial_number_isnull: None,
            archive_serial_number_lt: None,
            archive_serial_number_lte: None,
            checksum_icontains: None,
            checksum_iendswith: None,
            checksum_iexact: None,
            checksum_istartswith: None,
            content_icontains: None,
            content_iendswith: None,
            content_iexact: None,
            content_istartswith: None,
            correspondent_id: None,
            correspondent_id_in: None,
            correspondent_id_none: None,
            correspondent_isnull: None,
            correspondent_name_icontains: None,
            correspondent_name_iendswith: None,
            correspondent_name_iexact: None,
            correspondent_name_istartswith: None,
            created_date_gt: None,
            created_date_gte: None,
            created_date_lt: None,
            created_date_lte: None,
            created_day: None,
            created_gt: None,
            created_gte: None,
            created_lt: None,
            created_lte: None,
            created_month: None,
            created_year: None,
            custom_field_query: None,
            custom_fields_icontains: None,
            custom_fields_id_all: None,
            custom_fields_id_in: None,
            custom_fields_id_none: None,
            document_type_id: None,
            document_type_id_in: None,
            document_type_id_none: None,
            document_type_isnull: None,
            document_type_name_icontains: None,
            document_type_name_iendswith: None,
            document_type_name_iexact: None,
            document_type_name_istartswith: None,
            fields: None,
            has_custom_fields: None,
            id: None,
            id_in: None,
            is_in_inbox: None,
            is_tagged: None,
            mime_type: None,
            modified_date_gt: None,
            modified_date_gte: None,
            modified_date_lt: None,
            modified_date_lte: None,
            modified_day: None,
            modified_gt: None,
            modified_gte: None,
            modified_lt: None,
            modified_lte: None,
            modified_month: None,
            modified_year: None,
            ordering: None,
            original_filename_icontains: None,
            original_filename_iendswith: None,
            original_filename_iexact: None,
            original_filename_istartswith: None,
            owner_id: None,
            owner_id_in: None,
            owner_id_none: None,
            owner_isnull: None,
            page: None,
            page_size: None,
            search: None,
            shared_by_id: None,
            storage_path_id: None,
            storage_path_id_in: None,
            storage_path_id_none: None,
            storage_path_isnull: None,
            storage_path_name_icontains: None,
            storage_path_name_iendswith: None,
            storage_path_name_iexact: None,
            storage_path_name_istartswith: None,
            tags_id: None,
            tags_id_all: None,
            tags_id_in: None,
            tags_id_none: None,
            tags_name_icontains: None,
            tags_name_iendswith: None,
            tags_name_iexact: None,
            tags_name_istartswith: None,
            title_icontains: None,
            title_iendswith: None,
            title_iexact: None,
            title_istartswith: None,
            title_content: None,
        }
    }
}

impl List {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn added_date_gt(mut self, value: String) -> Self {
        self.added_date_gt = Some(value);
        self
    }

    #[must_use]
    pub fn added_date_gte(mut self, value: String) -> Self {
        self.added_date_gte = Some(value);
        self
    }

    #[must_use]
    pub fn added_date_lt(mut self, value: String) -> Self {
        self.added_date_lt = Some(value);
        self
    }

    #[must_use]
    pub fn added_date_lte(mut self, value: String) -> Self {
        self.added_date_lte = Some(value);
        self
    }

    #[must_use]
    pub fn added_day(mut self, value: f64) -> Self {
        self.added_day = Some(value);
        self
    }

    #[must_use]
    pub fn added_gt(mut self, value: String) -> Self {
        self.added_gt = Some(value);
        self
    }

    #[must_use]
    pub fn added_gte(mut self, value: String) -> Self {
        self.added_gte = Some(value);
        self
    }

    #[must_use]
    pub fn added_lt(mut self, value: String) -> Self {
        self.added_lt = Some(value);
        self
    }

    #[must_use]
    pub fn added_lte(mut self, value: String) -> Self {
        self.added_lte = Some(value);
        self
    }

    #[must_use]
    pub fn added_month(mut self, value: f64) -> Self {
        self.added_month = Some(value);
        self
    }

    #[must_use]
    pub fn added_year(mut self, value: f64) -> Self {
        self.added_year = Some(value);
        self
    }

    #[must_use]
    pub fn archive_serial_number(mut self, value: i32) -> Self {
        self.archive_serial_number = Some(value);
        self
    }

    #[must_use]
    pub fn archive_serial_number_gt(mut self, value: i32) -> Self {
        self.archive_serial_number_gt = Some(value);
        self
    }

    #[must_use]
    pub fn archive_serial_number_gte(mut self, value: i32) -> Self {
        self.archive_serial_number_gte = Some(value);
        self
    }

    #[must_use]
    pub fn archive_serial_number_isnull(mut self, value: bool) -> Self {
        self.archive_serial_number_isnull = Some(value);
        self
    }

    #[must_use]
    pub fn archive_serial_number_lt(mut self, value: i32) -> Self {
        self.archive_serial_number_lt = Some(value);
        self
    }

    #[must_use]
    pub fn archive_serial_number_lte(mut self, value: i32) -> Self {
        self.archive_serial_number_lte = Some(value);
        self
    }

    #[must_use]
    pub fn checksum_icontains(mut self, value: String) -> Self {
        self.checksum_icontains = Some(value);
        self
    }

    #[must_use]
    pub fn checksum_iendswith(mut self, value: String) -> Self {
        self.checksum_iendswith = Some(value);
        self
    }

    #[must_use]
    pub fn checksum_iexact(mut self, value: String) -> Self {
        self.checksum_iexact = Some(value);
        self
    }

    #[must_use]
    pub fn checksum_istartswith(mut self, value: String) -> Self {
        self.checksum_istartswith = Some(value);
        self
    }

    #[must_use]
    pub fn content_icontains(mut self, value: String) -> Self {
        self.content_icontains = Some(value);
        self
    }

    #[must_use]
    pub fn content_iendswith(mut self, value: String) -> Self {
        self.content_iendswith = Some(value);
        self
    }

    #[must_use]
    pub fn content_iexact(mut self, value: String) -> Self {
        self.content_iexact = Some(value);
        self
    }

    #[must_use]
    pub fn content_istartswith(mut self, value: String) -> Self {
        self.content_istartswith = Some(value);
        self
    }

    #[must_use]
    pub fn correspondent_id(mut self, value: i32) -> Self {
        self.correspondent_id = Some(value);
        self
    }

    #[must_use]
    pub fn correspondent_id_in(mut self, value: Vec<i32>) -> Self {
        self.correspondent_id_in = Some(value);
        self
    }

    #[must_use]
    pub fn correspondent_id_none(mut self, value: i32) -> Self {
        self.correspondent_id_none = Some(value);
        self
    }

    #[must_use]
    pub fn correspondent_isnull(mut self, value: bool) -> Self {
        self.correspondent_isnull = Some(value);
        self
    }

    #[must_use]
    pub fn correspondent_name_icontains(mut self, value: String) -> Self {
        self.correspondent_name_icontains = Some(value);
        self
    }

    #[must_use]
    pub fn correspondent_name_iendswith(mut self, value: String) -> Self {
        self.correspondent_name_iendswith = Some(value);
        self
    }

    #[must_use]
    pub fn correspondent_name_iexact(mut self, value: String) -> Self {
        self.correspondent_name_iexact = Some(value);
        self
    }

    #[must_use]
    pub fn correspondent_name_istartswith(mut self, value: String) -> Self {
        self.correspondent_name_istartswith = Some(value);
        self
    }

    #[must_use]
    pub fn created_date_gt(mut self, value: String) -> Self {
        self.created_date_gt = Some(value);
        self
    }

    #[must_use]
    pub fn created_date_gte(mut self, value: String) -> Self {
        self.created_date_gte = Some(value);
        self
    }

    #[must_use]
    pub fn created_date_lt(mut self, value: String) -> Self {
        self.created_date_lt = Some(value);
        self
    }

    #[must_use]
    pub fn created_date_lte(mut self, value: String) -> Self {
        self.created_date_lte = Some(value);
        self
    }

    #[must_use]
    pub fn created_day(mut self, value: f64) -> Self {
        self.created_day = Some(value);
        self
    }

    #[must_use]
    pub fn created_gt(mut self, value: String) -> Self {
        self.created_gt = Some(value);
        self
    }

    #[must_use]
    pub fn created_gte(mut self, value: String) -> Self {
        self.created_gte = Some(value);
        self
    }

    #[must_use]
    pub fn created_lt(mut self, value: String) -> Self {
        self.created_lt = Some(value);
        self
    }

    #[must_use]
    pub fn created_lte(mut self, value: String) -> Self {
        self.created_lte = Some(value);
        self
    }

    #[must_use]
    pub fn created_month(mut self, value: f64) -> Self {
        self.created_month = Some(value);
        self
    }

    #[must_use]
    pub fn created_year(mut self, value: f64) -> Self {
        self.created_year = Some(value);
        self
    }

    #[must_use]
    pub fn custom_field_query(mut self, value: String) -> Self {
        self.custom_field_query = Some(value);
        self
    }

    #[must_use]
    pub fn custom_fields_icontains(mut self, value: String) -> Self {
        self.custom_fields_icontains = Some(value);
        self
    }

    #[must_use]
    pub fn custom_fields_id_all(mut self, value: i32) -> Self {
        self.custom_fields_id_all = Some(value);
        self
    }

    #[must_use]
    pub fn custom_fields_id_in(mut self, value: i32) -> Self {
        self.custom_fields_id_in = Some(value);
        self
    }

    #[must_use]
    pub fn custom_fields_id_none(mut self, value: i32) -> Self {
        self.custom_fields_id_none = Some(value);
        self
    }

    #[must_use]
    pub fn document_type_id(mut self, value: i32) -> Self {
        self.document_type_id = Some(value);
        self
    }

    #[must_use]
    pub fn document_type_id_in(mut self, value: Vec<i32>) -> Self {
        self.document_type_id_in = Some(value);
        self
    }

    #[must_use]
    pub fn document_type_id_none(mut self, value: i32) -> Self {
        self.document_type_id_none = Some(value);
        self
    }

    #[must_use]
    pub fn document_type_isnull(mut self, value: bool) -> Self {
        self.document_type_isnull = Some(value);
        self
    }

    #[must_use]
    pub fn document_type_name_icontains(mut self, value: String) -> Self {
        self.document_type_name_icontains = Some(value);
        self
    }

    #[must_use]
    pub fn document_type_name_iendswith(mut self, value: String) -> Self {
        self.document_type_name_iendswith = Some(value);
        self
    }

    #[must_use]
    pub fn document_type_name_iexact(mut self, value: String) -> Self {
        self.document_type_name_iexact = Some(value);
        self
    }

    #[must_use]
    pub fn document_type_name_istartswith(mut self, value: String) -> Self {
        self.document_type_name_istartswith = Some(value);
        self
    }

    #[must_use]
    pub fn fields(mut self, value: Vec<String>) -> Self {
        self.fields = Some(value);
        self
    }

    #[must_use]
    pub fn has_custom_fields(mut self, value: bool) -> Self {
        self.has_custom_fields = Some(value);
        self
    }

    #[must_use]
    pub fn id(mut self, value: i32) -> Self {
        self.id = Some(value);
        self
    }

    #[must_use]
    pub fn id_in(mut self, value: Vec<i32>) -> Self {
        self.id_in = Some(value);
        self
    }

    #[must_use]
    pub fn is_in_inbox(mut self, value: bool) -> Self {
        self.is_in_inbox = Some(value);
        self
    }

    #[must_use]
    pub fn is_tagged(mut self, value: bool) -> Self {
        self.is_tagged = Some(value);
        self
    }

    #[must_use]
    pub fn mime_type(mut self, value: String) -> Self {
        self.mime_type = Some(value);
        self
    }

    #[must_use]
    pub fn modified_date_gt(mut self, value: String) -> Self {
        self.modified_date_gt = Some(value);
        self
    }

    #[must_use]
    pub fn modified_date_gte(mut self, value: String) -> Self {
        self.modified_date_gte = Some(value);
        self
    }

    #[must_use]
    pub fn modified_date_lt(mut self, value: String) -> Self {
        self.modified_date_lt = Some(value);
        self
    }

    #[must_use]
    pub fn modified_date_lte(mut self, value: String) -> Self {
        self.modified_date_lte = Some(value);
        self
    }

    #[must_use]
    pub fn modified_day(mut self, value: f64) -> Self {
        self.modified_day = Some(value);
        self
    }

    #[must_use]
    pub fn modified_gt(mut self, value: String) -> Self {
        self.modified_gt = Some(value);
        self
    }

    #[must_use]
    pub fn modified_gte(mut self, value: String) -> Self {
        self.modified_gte = Some(value);
        self
    }

    #[must_use]
    pub fn modified_lt(mut self, value: String) -> Self {
        self.modified_lt = Some(value);
        self
    }

    #[must_use]
    pub fn modified_lte(mut self, value: String) -> Self {
        self.modified_lte = Some(value);
        self
    }

    #[must_use]
    pub fn modified_month(mut self, value: f64) -> Self {
        self.modified_month = Some(value);
        self
    }

    #[must_use]
    pub fn modified_year(mut self, value: f64) -> Self {
        self.modified_year = Some(value);
        self
    }

    #[must_use]
    pub fn ordering(mut self, value: String) -> Self {
        self.ordering = Some(value);
        self
    }

    #[must_use]
    pub fn original_filename_icontains(mut self, value: String) -> Self {
        self.original_filename_icontains = Some(value);
        self
    }

    #[must_use]
    pub fn original_filename_iendswith(mut self, value: String) -> Self {
        self.original_filename_iendswith = Some(value);
        self
    }

    #[must_use]
    pub fn original_filename_iexact(mut self, value: String) -> Self {
        self.original_filename_iexact = Some(value);
        self
    }

    #[must_use]
    pub fn original_filename_istartswith(mut self, value: String) -> Self {
        self.original_filename_istartswith = Some(value);
        self
    }

    #[must_use]
    pub fn owner_id(mut self, value: i32) -> Self {
        self.owner_id = Some(value);
        self
    }

    #[must_use]
    pub fn owner_id_in(mut self, value: Vec<i32>) -> Self {
        self.owner_id_in = Some(value);
        self
    }

    #[must_use]
    pub fn owner_id_none(mut self, value: i32) -> Self {
        self.owner_id_none = Some(value);
        self
    }

    #[must_use]
    pub fn owner_isnull(mut self, value: bool) -> Self {
        self.owner_isnull = Some(value);
        self
    }

    #[must_use]
    pub fn page(mut self, value: i32) -> Self {
        self.page = Some(value);
        self
    }

    #[must_use]
    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }

    #[must_use]
    pub fn search(mut self, value: String) -> Self {
        self.search = Some(value);
        self
    }

    #[must_use]
    pub fn shared_by_id(mut self, value: bool) -> Self {
        self.shared_by_id = Some(value);
        self
    }

    #[must_use]
    pub fn storage_path_id(mut self, value: i32) -> Self {
        self.storage_path_id = Some(value);
        self
    }

    #[must_use]
    pub fn storage_path_id_in(mut self, value: Vec<i32>) -> Self {
        self.storage_path_id_in = Some(value);
        self
    }

    #[must_use]
    pub fn storage_path_id_none(mut self, value: i32) -> Self {
        self.storage_path_id_none = Some(value);
        self
    }

    #[must_use]
    pub fn storage_path_isnull(mut self, value: bool) -> Self {
        self.storage_path_isnull = Some(value);
        self
    }

    #[must_use]
    pub fn storage_path_name_icontains(mut self, value: String) -> Self {
        self.storage_path_name_icontains = Some(value);
        self
    }

    #[must_use]
    pub fn storage_path_name_iendswith(mut self, value: String) -> Self {
        self.storage_path_name_iendswith = Some(value);
        self
    }

    #[must_use]
    pub fn storage_path_name_iexact(mut self, value: String) -> Self {
        self.storage_path_name_iexact = Some(value);
        self
    }

    #[must_use]
    pub fn storage_path_name_istartswith(mut self, value: String) -> Self {
        self.storage_path_name_istartswith = Some(value);
        self
    }

    #[must_use]
    pub fn tags_id(mut self, value: i32) -> Self {
        self.tags_id = Some(value);
        self
    }

    #[must_use]
    pub fn tags_id_all(mut self, value: i32) -> Self {
        self.tags_id_all = Some(value);
        self
    }

    #[must_use]
    pub fn tags_id_in(mut self, value: i32) -> Self {
        self.tags_id_in = Some(value);
        self
    }

    #[must_use]
    pub fn tags_id_none(mut self, value: i32) -> Self {
        self.tags_id_none = Some(value);
        self
    }

    #[must_use]
    pub fn tags_name_icontains(mut self, value: String) -> Self {
        self.tags_name_icontains = Some(value);
        self
    }

    #[must_use]
    pub fn tags_name_iendswith(mut self, value: String) -> Self {
        self.tags_name_iendswith = Some(value);
        self
    }

    #[must_use]
    pub fn tags_name_iexact(mut self, value: String) -> Self {
        self.tags_name_iexact = Some(value);
        self
    }

    #[must_use]
    pub fn tags_name_istartswith(mut self, value: String) -> Self {
        self.tags_name_istartswith = Some(value);
        self
    }

    #[must_use]
    pub fn title_icontains(mut self, value: String) -> Self {
        self.title_icontains = Some(value);
        self
    }

    #[must_use]
    pub fn title_iendswith(mut self, value: String) -> Self {
        self.title_iendswith = Some(value);
        self
    }

    #[must_use]
    pub fn title_iexact(mut self, value: String) -> Self {
        self.title_iexact = Some(value);
        self
    }

    #[must_use]
    pub fn title_istartswith(mut self, value: String) -> Self {
        self.title_istartswith = Some(value);
        self
    }

    #[must_use]
    pub fn title_content(mut self, value: String) -> Self {
        self.title_content = Some(value);
        self
    }
}
