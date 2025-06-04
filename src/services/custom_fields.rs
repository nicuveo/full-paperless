use async_trait::async_trait;
use reqwest::Method;

use crate::body;
use crate::client::Client;
use crate::error::Result;
use crate::params;
use crate::response::Response;
use crate::schema::api::custom_fields::{Create, List, Patch};
use crate::schema::model::{CustomField, Paginated};

pub type Item = CustomField;

#[async_trait]
pub trait CustomFields {
    async fn list(&self, params: &List) -> Result<Response<Paginated<Item>>>;
    async fn create(&self, body: &Create) -> Result<Response<Item>>;
    async fn retrieve(&self, id: i32) -> Result<Response<Item>>;
    async fn update(&self, item: &Item) -> Result<Response<Item>>;
    async fn patch(&self, item: &mut Item, body: &Patch) -> Result<Response<()>>;
    async fn destroy(&self, item: Item) -> Result<Response<()>>;

    async fn previous_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>>>>;
    async fn next_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>>>>;
}

#[async_trait]
impl CustomFields for Client {
    async fn list(&self, params: &List) -> Result<Response<Paginated<Item>>> {
        let path = "/api/custom_fields/";
        let req = self.build_request(Method::GET, path, params, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn create(&self, body: &Create) -> Result<Response<Item>> {
        let path = "/api/custom_fields/";
        let req = self.build_request(Method::POST, path, params::NONE, body::json(body))?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn retrieve(&self, id: i32) -> Result<Response<Item>> {
        let path = format!("/api/custom_fields/{id}/");
        let req = self.build_request(Method::GET, &path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn update(&self, body: &Item) -> Result<Response<Item>> {
        let path = format!("/api/custom_fields/{}/", body.id);
        let body = Create::from(body);
        let req = self.build_request(Method::PUT, &path, params::NONE, body::json(&body))?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn patch(&self, item: &mut Item, body: &Patch) -> Result<Response<()>> {
        let path = format!("/api/custom_fields/{}/", item.id);
        let req = self.build_request(Method::PATCH, &path, params::NONE, body::json(body))?;
        let resp = Self::decode_json(self.send_request(req).await?).await?;
        Ok(resp.assign(item))
    }

    async fn destroy(&self, item: Item) -> Result<Response<()>> {
        let path = format!("/api/custom_fields/{}/", item.id);
        let req = self.build_request(Method::DELETE, &path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::ignore_content(resp)
    }

    async fn previous_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>>>> {
        Client::previous_page(self, current).await
    }

    async fn next_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>>>> {
        Client::next_page(self, current).await
    }
}
