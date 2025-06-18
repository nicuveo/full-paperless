use crate::clients::Client;
use crate::error::Result;
use crate::response::Response;
use crate::schema::api::profile::Patch;
use crate::schema::model;
use crate::utils::{Method, body, params};
use async_trait::async_trait;

type Item = model::Profile;

#[async_trait]
pub trait Profile<E = ()> {
    async fn retrieve(&self) -> Result<Response<Item, E>>;
    async fn patch(&self, body: &Patch) -> Result<Response<Item, E>>;
}

#[async_trait]
impl<C: Client> Profile<C::Extra> for C {
    async fn retrieve(&self) -> Result<Response<Item, C::Extra>> {
        let path = "/api/profile/";
        self.request_json(Method::GET, path, params::NONE, body::NONE)
            .await
    }

    async fn patch(&self, body: &Patch) -> Result<Response<Item, C::Extra>> {
        let path = "/api/profile/";
        self.request_json(Method::PATCH, path, params::NONE, Some(body))
            .await
    }
}
