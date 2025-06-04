use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct List {
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
    #[serde(rename = "expiration__date__gt")]
    pub expiration_date_gt: Option<String>,
    #[serde(rename = "expiration__date__gte")]
    pub expiration_date_gte: Option<String>,
    #[serde(rename = "expiration__date__lt")]
    pub expiration_date_lt: Option<String>,
    #[serde(rename = "expiration__date__lte")]
    pub expiration_date_lte: Option<String>,
    #[serde(rename = "expiration__day")]
    pub expiration_day: Option<f64>,
    #[serde(rename = "expiration__gt")]
    pub expiration_gt: Option<String>,
    #[serde(rename = "expiration__gte")]
    pub expiration_gte: Option<String>,
    #[serde(rename = "expiration__lt")]
    pub expiration_lt: Option<String>,
    #[serde(rename = "expiration__lte")]
    pub expiration_lte: Option<String>,
    #[serde(rename = "expiration__month")]
    pub expiration_month: Option<f64>,
    #[serde(rename = "expiration__year")]
    pub expiration_year: Option<f64>,
    pub ordering: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[must_use]
pub fn list() -> List {
    List::new()
}

impl List {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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
    pub fn expiration_date_gt(mut self, value: String) -> Self {
        self.expiration_date_gt = Some(value);
        self
    }

    #[must_use]
    pub fn expiration_date_gte(mut self, value: String) -> Self {
        self.expiration_date_gte = Some(value);
        self
    }

    #[must_use]
    pub fn expiration_date_lt(mut self, value: String) -> Self {
        self.expiration_date_lt = Some(value);
        self
    }

    #[must_use]
    pub fn expiration_date_lte(mut self, value: String) -> Self {
        self.expiration_date_lte = Some(value);
        self
    }

    #[must_use]
    pub fn expiration_day(mut self, value: f64) -> Self {
        self.expiration_day = Some(value);
        self
    }

    #[must_use]
    pub fn expiration_gt(mut self, value: String) -> Self {
        self.expiration_gt = Some(value);
        self
    }

    #[must_use]
    pub fn expiration_gte(mut self, value: String) -> Self {
        self.expiration_gte = Some(value);
        self
    }

    #[must_use]
    pub fn expiration_lt(mut self, value: String) -> Self {
        self.expiration_lt = Some(value);
        self
    }

    #[must_use]
    pub fn expiration_lte(mut self, value: String) -> Self {
        self.expiration_lte = Some(value);
        self
    }

    #[must_use]
    pub fn expiration_month(mut self, value: f64) -> Self {
        self.expiration_month = Some(value);
        self
    }

    #[must_use]
    pub fn expiration_year(mut self, value: f64) -> Self {
        self.expiration_year = Some(value);
        self
    }

    #[must_use]
    pub fn ordering(mut self, value: String) -> Self {
        self.ordering = Some(value);
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
}
