use crate::clients::Client;
use crate::error::Result;
use crate::response::Response;
use crate::schema::api::documents::{History, List, Patch};
use crate::schema::model::{
    Document, DocumentMetadata, LogEntry, Paginated, ShareLink, Suggestions,
};
use crate::utils::{Method, body, params};
use async_trait::async_trait;
use bytes::Bytes;

pub type Item = Document;

#[async_trait]
pub trait Documents<E = ()> {
    async fn list(&self, params: &List) -> Result<Response<Paginated<Item>, E>>;
    async fn retrieve(&self, id: i32) -> Result<Response<Item, E>>;
    async fn patch(&self, id: i32, body: &Patch) -> Result<Response<Item, E>>;
    async fn destroy(&self, id: i32) -> Result<Response<(), E>>;
    async fn preview(&self, id: i32) -> Result<Response<Bytes, E>>;

    async fn thumbnail(&self, id: i32) -> Result<Response<Bytes, E>>;
    async fn download(&self, id: i32, original: Option<bool>) -> Result<Response<Bytes, E>>;
    async fn history(&self, id: i32, params: &History) -> Result<Response<Paginated<LogEntry>, E>>;
    async fn metadata(&self, id: i32) -> Result<Response<DocumentMetadata, E>>;
    async fn share_links(&self, id: i32) -> Result<Response<Vec<ShareLink>, E>>;
    async fn sugestions(&self, id: i32) -> Result<Response<Suggestions, E>>;

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
impl<C: Client> Documents<C::Extra> for C {
    async fn list(&self, params: &List) -> Result<Response<Paginated<Item>, C::Extra>> {
        let path = "/api/document/";
        self.request_json(Method::GET, path, params, body::NONE)
            .await
    }

    async fn retrieve(&self, id: i32) -> Result<Response<Item, C::Extra>> {
        let path = format!("/api/document/{id}/");
        let params = vec![("full_perms", true)];
        self.request_json(Method::GET, &path, &params, body::NONE)
            .await
    }

    async fn patch(&self, id: i32, body: &Patch) -> Result<Response<Item, C::Extra>> {
        let path = format!("/api/document/{id}/");
        self.request_json(Method::PATCH, &path, params::NONE, Some(body))
            .await
    }

    async fn destroy(&self, id: i32) -> Result<Response<(), C::Extra>> {
        let path = format!("/api/document/{id}/");
        self.request_unit(Method::DELETE, &path, params::NONE, body::NONE)
            .await
    }

    async fn preview(&self, id: i32) -> Result<Response<Bytes, C::Extra>> {
        let path = format!("/api/document/{id}/preview");
        self.request_bytes(Method::GET, &path, params::NONE, body::NONE)
            .await
    }

    async fn thumbnail(&self, id: i32) -> Result<Response<Bytes, C::Extra>> {
        let path = format!("/api/document/{id}/thumb");
        self.request_bytes(Method::GET, &path, params::NONE, body::NONE)
            .await
    }

    async fn download(&self, id: i32, original: Option<bool>) -> Result<Response<Bytes, C::Extra>> {
        let path = format!("/api/document/{id}/download");
        let params = original.map(|o| vec![("original", o)]);
        self.request_bytes(Method::GET, &path, &params, body::NONE)
            .await
    }

    async fn history(
        &self,
        id: i32,
        params: &History,
    ) -> Result<Response<Paginated<LogEntry>, C::Extra>> {
        let path = format!("/api/document/{id}/history");
        self.request_json(Method::GET, &path, params, body::NONE)
            .await
    }

    async fn metadata(&self, id: i32) -> Result<Response<DocumentMetadata, C::Extra>> {
        let path = format!("/api/document/{id}/metadata");
        self.request_json(Method::GET, &path, params::NONE, body::NONE)
            .await
    }

    async fn share_links(&self, id: i32) -> Result<Response<Vec<ShareLink>, C::Extra>> {
        let path = format!("/api/document/{id}/share_links");
        self.request_json(Method::GET, &path, params::NONE, body::NONE)
            .await
    }

    async fn sugestions(&self, id: i32) -> Result<Response<Suggestions, C::Extra>> {
        let path = format!("/api/document/{id}/suggestions");
        self.request_json(Method::GET, &path, params::NONE, body::NONE)
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
