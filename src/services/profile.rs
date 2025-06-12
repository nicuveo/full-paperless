use crate::clients::Client;
use crate::error::Result;
use crate::response::Response;
use crate::schema::api::profile::Patch;
use crate::schema::model;
use crate::utils::{Method, body, params};
use async_trait::async_trait;

type Item = model::Profile;

#[async_trait]
pub trait Profile<E>: Sized {
    fn profile(&self) -> &impl Profile<E> {
        self
    }

    async fn retrieve(&self) -> Result<Response<Item, E>>;
    async fn patch(&self, body: &Patch) -> Result<Response<Item, E>>;
}

#[async_trait]
impl<C: Client> Profile<C::Extra> for C {
    async fn retrieve(&self) -> Result<Response<Item, C::Extra>> {
        let path = "/api/profile/";
        let resp = self
            .send(Method::GET, path, params::NONE, body::NONE)
            .await?;
        Self::decode_json(resp).await
    }

    async fn patch(&self, body: &Patch) -> Result<Response<Item, C::Extra>> {
        let path = "/api/profile/";
        let resp = self
            .send(Method::PATCH, path, params::NONE, Some(body))
            .await?;
        Self::decode_json(resp).await
    }
}
