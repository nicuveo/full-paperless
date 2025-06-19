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
        self.request_json(Method::GET, path, params, body::NONE)
            .await
    }

    async fn create(&self, body: &Create) -> Result<Response<Item, C::Extra>> {
        let path = "/api/correspondents/";
        self.request_json(Method::POST, path, params::NONE, Some(body))
            .await
    }

    async fn retrieve(&self, id: i32) -> Result<Response<Item, C::Extra>> {
        let path = format!("/api/correspondents/{id}/");
        let params = vec![("full_perms", true)];
        self.request_json(Method::GET, &path, &params, body::NONE)
            .await
    }

    async fn patch(&self, id: i32, body: &Patch) -> Result<Response<Item, C::Extra>> {
        let path = format!("/api/correspondents/{id}/");
        let params = vec![("full_perms", true)];
        self.request_json(Method::PATCH, &path, &params, Some(body))
            .await
    }

    async fn destroy(&self, id: i32) -> Result<Response<(), C::Extra>> {
        let path = format!("/api/correspondents/{id}/");
        self.request_unit(Method::DELETE, &path, params::NONE, body::NONE)
            .await
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
