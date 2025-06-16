use crate::clients::Client;
use crate::error::Result;
use crate::response::Response;
use crate::schema::api::documents::{Create, History, List, Patch};
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
    async fn update(&self, item: &Item) -> Result<Response<Item, E>>;
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
        let resp = self.send(Method::GET, path, params, body::NONE).await?;
        Self::decode_json(resp).await
    }

    async fn retrieve(&self, id: i32) -> Result<Response<Item, C::Extra>> {
        let path = format!("/api/document/{id}/");
        let params = vec![("full_perms", true)];
        let resp = self.send(Method::GET, &path, &params, body::NONE).await?;
        Self::decode_json(resp).await
    }

    async fn update(&self, body: &Item) -> Result<Response<Item, C::Extra>> {
        let path = format!("/api/document/{}/", body.id);
        let body = Create::from(body);
        let resp = self
            .send(Method::PUT, &path, params::NONE, Some(&body))
            .await?;
        Self::decode_json(resp).await
    }

    async fn patch(&self, id: i32, body: &Patch) -> Result<Response<Item, C::Extra>> {
        let path = format!("/api/document/{id}/");
        let resp = self
            .send(Method::PATCH, &path, params::NONE, Some(body))
            .await?;
        Self::decode_json(resp).await
    }

    async fn destroy(&self, id: i32) -> Result<Response<(), C::Extra>> {
        let path = format!("/api/document/{id}/");
        let resp = self
            .send(Method::DELETE, &path, params::NONE, body::NONE)
            .await?;
        Self::ignore_content(resp)
    }

    async fn preview(&self, id: i32) -> Result<Response<Bytes, C::Extra>> {
        let path = format!("/api/document/{id}/preview");
        let resp = self
            .send(Method::GET, &path, params::NONE, body::NONE)
            .await?;
        Self::decode_bytes(resp).await
    }

    async fn thumbnail(&self, id: i32) -> Result<Response<Bytes, C::Extra>> {
        let path = format!("/api/document/{id}/thumb");
        let resp = self
            .send(Method::GET, &path, params::NONE, body::NONE)
            .await?;
        Self::decode_bytes(resp).await
    }

    async fn download(&self, id: i32, original: Option<bool>) -> Result<Response<Bytes, C::Extra>> {
        let path = format!("/api/document/{id}/download");
        let params = original.map(|o| vec![("original", o)]);
        let resp = self.send(Method::GET, &path, &params, body::NONE).await?;
        Self::decode_bytes(resp).await
    }

    async fn history(
        &self,
        id: i32,
        params: &History,
    ) -> Result<Response<Paginated<LogEntry>, C::Extra>> {
        let path = format!("/api/document/{id}/history");
        let resp = self.send(Method::GET, &path, params, body::NONE).await?;
        Self::decode_json(resp).await
    }

    async fn metadata(&self, id: i32) -> Result<Response<DocumentMetadata, C::Extra>> {
        let path = format!("/api/document/{id}/metadata");
        let resp = self
            .send(Method::GET, &path, params::NONE, body::NONE)
            .await?;
        Self::decode_json(resp).await
    }

    async fn share_links(&self, id: i32) -> Result<Response<Vec<ShareLink>, C::Extra>> {
        let path = format!("/api/document/{id}/share_links");
        let resp = self
            .send(Method::GET, &path, params::NONE, body::NONE)
            .await?;
        Self::decode_json(resp).await
    }

    async fn sugestions(&self, id: i32) -> Result<Response<Suggestions, C::Extra>> {
        let path = format!("/api/document/{id}/suggestions");
        let resp = self
            .send(Method::GET, &path, params::NONE, body::NONE)
            .await?;
        Self::decode_json(resp).await
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
