use async_trait::async_trait;
use reqwest::Method;

use crate::body;
use crate::client::Client;
use crate::error::Result;
use crate::params;
use crate::response::Response;
use crate::schema::api::config::{Create, Patch};
use crate::schema::model::ApplicationConfiguration;

pub type Item = ApplicationConfiguration;

#[async_trait]
pub trait Config {
    async fn list(&self) -> Result<Response<Vec<Item>>>;
    async fn create(&self, body: &Create) -> Result<Response<Item>>;
    async fn retrieve(&self, id: i32) -> Result<Response<Item>>;
    async fn update(&self, item: &Item) -> Result<Response<Item>>;
    async fn patch(&self, item: &mut Item, body: &Patch) -> Result<Response<()>>;
    async fn destroy(&self, item: Item) -> Result<Response<()>>;
}

#[async_trait]
impl Config for Client {
    async fn list(&self) -> Result<Response<Vec<Item>>> {
        let path = "/api/config/";
        let req = self.build_request(Method::GET, path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn create(&self, body: &Create) -> Result<Response<Item>> {
        let path = "/api/config/";
        let req = self.build_request(Method::POST, path, params::NONE, body::json(body))?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn retrieve(&self, id: i32) -> Result<Response<Item>> {
        let path = format!("/api/config/{id}/");
        let req = self.build_request(Method::GET, &path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn update(&self, body: &Item) -> Result<Response<Item>> {
        let path = format!("/api/config/{}/", body.id);
        let body = Create::from(body);
        let req = self.build_request(Method::PUT, &path, params::NONE, body::json(&body))?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn patch(&self, item: &mut Item, body: &Patch) -> Result<Response<()>> {
        let path = format!("/api/config/{}/", item.id);
        let req = self.build_request(Method::PATCH, &path, params::NONE, body::json(body))?;
        let resp = Self::decode_json(self.send_request(req).await?).await?;
        Ok(resp.assign(item))
    }

    async fn destroy(&self, item: Item) -> Result<Response<()>> {
        let path = format!("/api/config/{}/", item.id);
        let req = self.build_request(Method::DELETE, &path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::ignore_content(resp)
    }
}
