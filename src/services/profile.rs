use async_trait::async_trait;
use reqwest::Method;

use crate::body;
use crate::client::Client;
use crate::error::Result;
use crate::params;
use crate::response::Response;
use crate::schema::api::profile::Patch;
use crate::schema::model;

type Item = model::Profile;

#[async_trait]
pub trait Profile {
    async fn retrieve(&self) -> Result<Response<Item>>;
    async fn patch(&self, body: &Patch) -> Result<Response<Item>>;
}

#[async_trait]
impl Profile for Client {
    async fn retrieve(&self) -> Result<Response<Item>> {
        let path = "/api/profile/";
        let req = self.build_request(Method::GET, path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn patch(&self, body: &Patch) -> Result<Response<Item>> {
        let path = "/api/profile/";
        let req = self.build_request(Method::PATCH, path, params::NONE, body::json(body))?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }
}
