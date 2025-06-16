use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::clients::Client;
use crate::error::Result;
use crate::response::Response;
use crate::schema::api::tasks::{Create, List};
use crate::schema::model::TaskView;
use crate::utils::{Method, body, params};

pub type Item = TaskView;

#[async_trait]
pub trait Tasks<E = ()> {
    async fn list(&self, params: &List) -> Result<Response<Vec<Item>, E>>;
    async fn retrieve(&self, id: i32) -> Result<Response<Item, E>>;
    async fn run(&self, body: &Create) -> Result<Response<Item, E>>;
    async fn acknowledge(&self, body: &[i32]) -> Result<Response<Vec<i32>, E>>;
}

#[async_trait]
impl<C: Client> Tasks<C::Extra> for C {
    async fn list(&self, params: &List) -> Result<Response<Vec<Item>, C::Extra>> {
        let path = "/api/tasks/";
        let resp = self.send(Method::GET, path, params, body::NONE).await?;
        Self::decode_json(resp).await
    }

    async fn retrieve(&self, id: i32) -> Result<Response<Item, C::Extra>> {
        let path = format!("/api/tasks/{id}/");
        let resp = self
            .send(Method::GET, &path, params::NONE, body::NONE)
            .await?;
        Self::decode_json(resp).await
    }

    async fn run(&self, body: &Create) -> Result<Response<Item, C::Extra>> {
        let path = "/api/tasks/run/";
        let resp = self
            .send(Method::PUT, path, params::NONE, Some(body))
            .await?;
        Self::decode_json(resp).await
    }

    async fn acknowledge(&self, body: &[i32]) -> Result<Response<Vec<i32>, C::Extra>> {
        let path = "/api/tasks/acknowledge/";
        let body = AcknowledgeInput { tasks: body };
        let resp = self
            .send(Method::PUT, path, params::NONE, Some(&body))
            .await?;
        let mut resp: Response<AcknowledgeOutput, C::Extra> = Self::decode_json(resp).await?;
        let value = std::mem::take(&mut resp.value.result);
        Ok(resp.replace(value))
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
