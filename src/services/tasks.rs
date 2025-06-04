use async_trait::async_trait;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::body;
use crate::client::Client;
use crate::error::Result;
use crate::params;
use crate::response::Response;
use crate::schema::api::tasks::{Create, List};
use crate::schema::model::TaskView;

pub type Item = TaskView;

#[async_trait]
pub trait Tasks {
    async fn list(&self, params: &List) -> Result<Response<Vec<Item>>>;
    async fn retrieve(&self, id: i32) -> Result<Response<Item>>;
    async fn run(&self, body: &Create) -> Result<Response<Item>>;
    async fn acknowledge(&self, body: &[i32]) -> Result<Response<Vec<i32>>>;
}

#[async_trait]
impl Tasks for Client {
    async fn list(&self, params: &List) -> Result<Response<Vec<Item>>> {
        let path = "/api/tasks/";
        let req = self.build_request(Method::GET, path, params, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn retrieve(&self, id: i32) -> Result<Response<Item>> {
        let path = format!("/api/tasks/{id}/");
        let req = self.build_request(Method::GET, &path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn run(&self, body: &Create) -> Result<Response<Item>> {
        let path = "/api/tasks/run/";
        let req = self.build_request(Method::PUT, path, params::NONE, body::json(body))?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn acknowledge(&self, body: &[i32]) -> Result<Response<Vec<i32>>> {
        let path = "/api/tasks/acknowledge/";
        let body = AcknowledgeInput { tasks: body };
        let req = self.build_request(Method::PUT, path, params::NONE, body::json(&body))?;
        let resp = self.send_request(req).await?;
        let resp: Response<AcknowledgeOutput> = Self::decode_json(resp).await?;
        Ok(Response {
            value: resp.value.result,
            status: resp.status,
            headers: resp.headers,
            content_type: resp.content_type,
        })
    }
}

#[derive(Serialize)]
struct AcknowledgeInput<'a> {
    tasks: &'a [i32],
}

#[derive(Deserialize)]
struct AcknowledgeOutput {
    result: Vec<i32>,
}
