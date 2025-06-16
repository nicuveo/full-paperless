use crate::clients::Client;
use crate::error::Result;
use crate::response::Response;
use crate::schema::api::correspondents::{Create, List, Patch};
use crate::schema::model::{Correspondent, Paginated};
use crate::utils::{Method, body, params};
use async_trait::async_trait;

pub type Item = Correspondent;

#[async_trait]
pub trait Correspondents<E> {
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
impl<C: Client> Correspondents<C::Extra> for C {
    async fn list(&self, params: &List) -> Result<Response<Paginated<Item>, C::Extra>> {
        let path = "/api/correspondents/";
        let resp = self.send(Method::GET, path, params, body::NONE).await?;
        Self::decode_json(resp).await
    }

    async fn create(&self, body: &Create) -> Result<Response<Item, C::Extra>> {
        let path = "/api/correspondents/";
        let resp = self
            .send(Method::POST, path, params::NONE, Some(body))
            .await?;
        let id = Self::decode_id(resp).await?;
        // required fields are missing in the response to the create request
        self.retrieve(id).await
    }

    async fn retrieve(&self, id: i32) -> Result<Response<Item, C::Extra>> {
        let path = format!("/api/correspondents/{id}/");
        let params = vec![("full_perms", true)];
        let resp = self.send(Method::GET, &path, &params, body::NONE).await?;
        Self::decode_json(resp).await
    }

    async fn update(&self, body: &Item) -> Result<Response<Item, C::Extra>> {
        let path = format!("/api/correspondents/{}/", body.id);
        let params = vec![("full_perms", true)];
        let body = Create::from(body);
        let resp = self.send(Method::PUT, &path, &params, Some(&body)).await?;
        Self::decode_json(resp).await
    }

    async fn patch(&self, id: i32, body: &Patch) -> Result<Response<Item, C::Extra>> {
        let path = format!("/api/correspondents/{id}/");
        let params = vec![("full_perms", true)];
        let resp = self.send(Method::PATCH, &path, &params, Some(body)).await?;
        Self::decode_json(resp).await
    }

    async fn destroy(&self, id: i32) -> Result<Response<(), C::Extra>> {
        let path = format!("/api/correspondents/{id}/");
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
