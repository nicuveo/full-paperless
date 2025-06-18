use async_trait::async_trait;

use crate::clients::Client;
use crate::error::Result;
use crate::response::Response;
use crate::schema::api::config::{Create, Patch};
use crate::schema::model::ApplicationConfiguration;
use crate::utils::{Method, body, params};

pub type Item = ApplicationConfiguration;

#[async_trait]
pub trait Config<E = ()> {
    async fn create(&self, body: &Create) -> Result<Response<Item, E>>;
    async fn retrieve(&self) -> Result<Response<Item, E>>;
    async fn patch(&self, body: &Patch) -> Result<Response<Item, E>>;
    async fn destroy(&self) -> Result<Response<(), E>>;
}

#[async_trait]
impl<C: Client> Config<C::Extra> for C {
    async fn create(&self, body: &Create) -> Result<Response<Item, C::Extra>> {
        self.request_json(Method::POST, "/api/config/", params::NONE, Some(body))
            .await
    }

    async fn retrieve(&self) -> Result<Response<Item, C::Extra>> {
        self.request_json(Method::GET, "/api/config/1/", params::NONE, body::NONE)
            .await
    }

    async fn patch(&self, body: &Patch) -> Result<Response<Item, C::Extra>> {
        self.request_json(Method::PATCH, "/api/config/1/", params::NONE, Some(body))
            .await
    }

    async fn destroy(&self) -> Result<Response<(), C::Extra>> {
        self.request_unit(Method::DELETE, "/api/config/1/", params::NONE, body::NONE)
            .await
    }
}
