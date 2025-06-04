use async_trait::async_trait;
use bytes::Bytes;
use reqwest::Method;

use crate::body;
use crate::client::Client;
use crate::error::Result;
use crate::params;
use crate::response::Response;
use crate::schema::api::documents::{Create, History, List, Patch};
use crate::schema::model::{
    Document, DocumentMetadata, LogEntry, Paginated, ShareLink, Suggestions,
};

pub type Item = Document;

#[async_trait]
pub trait Documents {
    async fn list(&self, params: &List) -> Result<Response<Paginated<Item>>>;
    async fn retrieve(&self, id: i32) -> Result<Response<Item>>;
    async fn update(&self, item: &Item) -> Result<Response<Item>>;
    async fn patch(&self, item: &mut Item, body: &Patch) -> Result<Response<()>>;
    async fn destroy(&self, item: Item) -> Result<Response<()>>;
    async fn preview(&self, id: i32) -> Result<Response<Bytes>>;

    async fn thumbnail(&self, id: i32) -> Result<Response<Bytes>>;
    async fn download(&self, id: i32, original: Option<bool>) -> Result<Response<Bytes>>;
    async fn history(&self, id: i32, params: &History) -> Result<Response<Paginated<LogEntry>>>;
    async fn metadata(&self, id: i32) -> Result<Response<DocumentMetadata>>;
    async fn share_links(&self, id: i32) -> Result<Response<Vec<ShareLink>>>;
    async fn sugestions(&self, id: i32) -> Result<Response<Suggestions>>;

    async fn previous_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>>>>;
    async fn next_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>>>>;
}

#[async_trait]
impl Documents for Client {
    async fn list(&self, params: &List) -> Result<Response<Paginated<Item>>> {
        let path = "/api/document/";
        let req = self.build_request(Method::GET, path, params, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn retrieve(&self, id: i32) -> Result<Response<Item>> {
        let path = format!("/api/document/{id}/");
        let params = vec![("full_perms", true)];
        let req = self.build_request(Method::GET, &path, &params, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn update(&self, body: &Item) -> Result<Response<Item>> {
        let path = format!("/api/document/{}/", body.id);
        let body = Create::from(body);
        let req = self.build_request(Method::PUT, &path, params::NONE, body::json(&body))?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn patch(&self, item: &mut Item, body: &Patch) -> Result<Response<()>> {
        let path = format!("/api/document/{}/", item.id);
        let req = self.build_request(Method::PATCH, &path, params::NONE, body::json(body))?;
        let resp = Self::decode_json(self.send_request(req).await?).await?;
        Ok(resp.assign(item))
    }

    async fn destroy(&self, item: Item) -> Result<Response<()>> {
        let path = format!("/api/document/{}/", item.id);
        let req = self.build_request(Method::DELETE, &path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::ignore_content(resp)
    }

    async fn preview(&self, id: i32) -> Result<Response<Bytes>> {
        let path = format!("/api/document/{id}/preview");
        let req = self.build_request(Method::GET, &path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_bytes(resp).await
    }

    async fn thumbnail(&self, id: i32) -> Result<Response<Bytes>> {
        let path = format!("/api/document/{id}/thumb");
        let req = self.build_request(Method::GET, &path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_bytes(resp).await
    }

    async fn download(&self, id: i32, original: Option<bool>) -> Result<Response<Bytes>> {
        let path = format!("/api/document/{id}/download");
        let params = original.map(|o| vec![("original", o)]);
        let req = self.build_request(Method::GET, &path, &params, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_bytes(resp).await
    }

    async fn history(&self, id: i32, params: &History) -> Result<Response<Paginated<LogEntry>>> {
        let path = format!("/api/document/{id}/history");
        let req = self.build_request(Method::GET, &path, params, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn metadata(&self, id: i32) -> Result<Response<DocumentMetadata>> {
        let path = format!("/api/document/{id}/metadata");
        let req = self.build_request(Method::GET, &path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn share_links(&self, id: i32) -> Result<Response<Vec<ShareLink>>> {
        let path = format!("/api/document/{id}/share_links");
        let req = self.build_request(Method::GET, &path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn sugestions(&self, id: i32) -> Result<Response<Suggestions>> {
        let path = format!("/api/document/{id}/suggestions");
        let req = self.build_request(Method::GET, &path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn previous_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>>>> {
        Client::previous_page(self, current).await
    }

    async fn next_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>>>> {
        Client::next_page(self, current).await
    }
}
