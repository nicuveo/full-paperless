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
    async fn list(&self) -> Result<Response<Vec<Item>, E>>;
    async fn create(&self, body: &Create) -> Result<Response<Item, E>>;
    async fn retrieve(&self, id: i32) -> Result<Response<Item, E>>;
    async fn update(&self, item: &Item) -> Result<Response<Item, E>>;
    async fn patch(&self, id: i32, body: &Patch) -> Result<Response<Item, E>>;
    async fn destroy(&self, id: i32) -> Result<Response<(), E>>;
}

#[async_trait]
impl<C: Client> Config<C::Extra> for C {
    async fn list(&self) -> Result<Response<Vec<Item>, C::Extra>> {
        Self::decode_json(
            self.send(Method::GET, "/api/config/", params::NONE, body::NONE)
                .await?,
        )
        .await
    }

    async fn create(&self, body: &Create) -> Result<Response<Item, C::Extra>> {
        Self::decode_json(
            self.send(Method::POST, "/api/config/", params::NONE, Some(body))
                .await?,
        )
        .await
    }

    async fn retrieve(&self, id: i32) -> Result<Response<Item, C::Extra>> {
        Self::decode_json(
            self.send(
                Method::GET,
                &format!("/api/config/{id}/"),
                params::NONE,
                body::NONE,
            )
            .await?,
        )
        .await
    }

    async fn update(&self, body: &Item) -> Result<Response<Item, C::Extra>> {
        Self::decode_json(
            self.send(
                Method::PUT,
                &format!("/api/config/{}/", body.id),
                params::NONE,
                Some(&Create::from(body)),
            )
            .await?,
        )
        .await
    }

    async fn patch(&self, id: i32, body: &Patch) -> Result<Response<Item, C::Extra>> {
        Self::decode_json(
            self.send(
                Method::PATCH,
                &format!("/api/config/{id}/"),
                params::NONE,
                Some(body),
            )
            .await?,
        )
        .await
    }

    async fn destroy(&self, id: i32) -> Result<Response<(), C::Extra>> {
        Self::ignore_content(
            self.send(
                Method::DELETE,
                &format!("/api/config/{id}/"),
                params::NONE,
                body::NONE,
            )
            .await?,
        )
    }
}
