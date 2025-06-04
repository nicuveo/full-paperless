use async_trait::async_trait;
use reqwest::Method;

use crate::body;
use crate::client::Client;
use crate::error::Result;
use crate::params;
use crate::response::Response;

#[async_trait]
pub trait Logs {
    async fn list(&self) -> Result<Response<Vec<String>>>;
    async fn retrieve(&self, log_type: &str) -> Result<Response<Vec<String>>>;
}

#[async_trait]
impl Logs for Client {
    async fn list(&self) -> Result<Response<Vec<String>>> {
        let path = "/api/logs/";
        let req = self.build_request(Method::GET, path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn retrieve(&self, log_type: &str) -> Result<Response<Vec<String>>> {
        let path = format!("/api/logs/{log_type}/");
        let req = self.build_request(Method::GET, &path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }
}
