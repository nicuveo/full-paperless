use crate::clients::Client;
use crate::error::Result;
use crate::response::Response;
use crate::schema::api::custom_fields::{Create, List, Patch};
use crate::schema::model::{CustomField, Paginated};
use crate::utils::{Method, body, params};
use async_trait::async_trait;

pub type Item = CustomField;

#[async_trait]
pub trait CustomFields<E = ()> {
    async fn list(&self, params: &List) -> Result<Response<Paginated<Item>, E>>;
    async fn create(&self, body: &Create) -> Result<Response<Item, E>>;
    async fn retrieve(&self, id: i32) -> Result<Response<Item, E>>;
    async fn update(&self, item: &Item) -> Result<Response<Item, E>>;
    async fn patch(&self, id: i32, body: &Patch) -> Result<Response<Item, E>>;
    async fn destroy(&self, id: i32) -> Result<Response<(), E>>;

    async fn previous_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>, E>>>;
    async fn next_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>, E>>>;
}

#[async_trait]
impl<C: Client> CustomFields<C::Extra> for C {
    async fn list(&self, params: &List) -> Result<Response<Paginated<Item>, C::Extra>> {
        let path = "/api/custom_fields/";
        let resp = self.send(Method::GET, path, params, body::NONE).await?;
        Self::decode_json(resp).await
    }

    async fn create(&self, body: &Create) -> Result<Response<Item, C::Extra>> {
        let path = "/api/custom_fields/";
        let resp = self
            .send(Method::POST, path, params::NONE, Some(body))
            .await?;
        Self::decode_json(resp).await
    }

    async fn retrieve(&self, id: i32) -> Result<Response<Item, C::Extra>> {
        let path = format!("/api/custom_fields/{id}/");
        let resp = self
            .send(Method::GET, &path, params::NONE, body::NONE)
            .await?;
        Self::decode_json(resp).await
    }

    async fn update(&self, body: &Item) -> Result<Response<Item, C::Extra>> {
        let path = format!("/api/custom_fields/{}/", body.id);
        let body = Create::from(body);
        let resp = self
            .send(Method::PUT, &path, params::NONE, Some(&body))
            .await?;
        Self::decode_json(resp).await
    }

    async fn patch(&self, id: i32, body: &Patch) -> Result<Response<Item, C::Extra>> {
        let path = format!("/api/custom_fields/{id}/");
        let resp = self
            .send(Method::PATCH, &path, params::NONE, Some(body))
            .await?;
        Self::decode_json(resp).await
    }

    async fn destroy(&self, id: i32) -> Result<Response<(), C::Extra>> {
        let path = format!("/api/custom_fields/{id}/");
        let resp = self
            .send(Method::DELETE, &path, params::NONE, body::NONE)
            .await?;
        Self::ignore_content(resp)
    }
    async fn previous_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>, C::Extra>>> {
        C::previous_page(self, current).await
    }

    async fn next_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>, C::Extra>>> {
        C::next_page(self, current).await
    }
}
