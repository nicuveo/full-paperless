use async_trait::async_trait;
use reqwest::Method;

use crate::body;
use crate::client::Client;
use crate::error::Result;
use crate::params;
use crate::response::Response;
use crate::schema::api::users::{Create, List, Patch};
use crate::schema::model::{Paginated, User};

pub type Item = User;

#[async_trait]
pub trait Users {
    async fn list(&self, params: &List) -> Result<Response<Paginated<Item>>>;
    async fn create(&self, body: &Create) -> Result<Response<Item>>;
    async fn retrieve(&self, id: i32) -> Result<Response<Item>>;
    async fn update(&self, body: &Item) -> Result<Response<Item>>;
    async fn patch(&self, id: i32, body: &Patch) -> Result<Response<Item>>;
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
impl Users for Client {
    async fn list(&self, params: &List) -> Result<Response<Paginated<Item>>> {
        let path = "/api/users/";
        let req = self.build_request(Method::GET, path, params, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn create(&self, body: &Create) -> Result<Response<Item>> {
        let path = "/api/users/";
        let req = self.build_request(Method::POST, path, params::NONE, body::json(body))?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn retrieve(&self, id: i32) -> Result<Response<Item>> {
        let path = format!("/api/users/{id}/");
        let req = self.build_request(Method::GET, &path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn update(&self, body: &Item) -> Result<Response<Item>> {
        let path = format!("/api/users/{}/", body.id);
        let body = Create::from(body);
        let req = self.build_request(Method::PUT, &path, params::NONE, body::json(&body))?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn patch(&self, id: i32, body: &Patch) -> Result<Response<Item>> {
        let path = format!("/api/users/{id}/");
        let req = self.build_request(Method::PATCH, &path, params::NONE, body::json(body))?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn destroy(&self, item: Item) -> Result<Response<()>> {
        let path = format!("/api/users/{}/", item.id);
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
