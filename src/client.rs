use bytes::Bytes;
use reqwest::{Method, Request, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

use crate::authentication::Auth;
use crate::body;
use crate::error::{Error, Result};
use crate::response::Response;
use crate::schema::model::Paginated;
use crate::services;

#[derive(Debug, Clone)]
pub struct Client {
    inner: reqwest::Client,
    server_url: String,
    auth: Auth,
    additional_headers: Vec<(String, String)>,
}

impl Client {
    #[must_use]
    pub fn new(server_url: String, auth: Auth) -> Self {
        Self {
            inner: reqwest::Client::new(),
            server_url,
            auth,
            additional_headers: vec![],
        }
    }

    #[must_use]
    pub fn with_headers(server_url: String, auth: Auth, headers: Vec<(String, String)>) -> Self {
        Self {
            inner: reqwest::Client::new(),
            server_url,
            auth,
            additional_headers: headers,
        }
    }

    #[must_use]
    pub fn additional_headers(&self) -> &[(String, String)] {
        &self.additional_headers
    }

    #[must_use]
    pub fn additional_headers_mut(&mut self) -> &mut Vec<(String, String)> {
        &mut self.additional_headers
    }

    ////////////////////////////////////////////////////////////////////////////
    // network

    #[must_use]
    pub fn inner_client(&self) -> &reqwest::Client {
        &self.inner
    }

    pub fn build_request<P, B>(
        &self,
        method: Method,
        endpoint: &str,
        params: &P,
        body_func: B,
    ) -> Result<Request>
    where
        B: FnOnce(RequestBuilder) -> RequestBuilder,
        P: Serialize,
    {
        let uri = format!("{}{endpoint}", self.server_url);
        let mut request = body_func(
            self.inner
                .request(method, &uri)
                .header(reqwest::header::ACCEPT, "application/json; version=9")
                .header(reqwest::header::AUTHORIZATION, self.auth.header_value())
                .query(params),
        );
        for (header_name, header_value) in &self.additional_headers {
            request = request.header(header_name, header_value);
        }
        request.build().map_err(Error::RequestBuild)
    }

    pub async fn send_request(&self, request: Request) -> Result<reqwest::Response> {
        let resp = self
            .inner
            .execute(request)
            .await
            .map_err(Error::RequestSend)?;
        let status = resp.status();

        if status.is_success() {
            Ok(resp)
        } else {
            let content = resp.text().await.map_err(Error::ResponseBody)?;
            let content =
                serde_json::from_str(&content).unwrap_or(serde_json::Value::String(content));
            Err(Error::Application { status, content })
        }
    }

    pub async fn decode_json<R>(resp: reqwest::Response) -> Result<Response<R>>
    where
        R: for<'a> Deserialize<'a>,
    {
        let status = resp.status();
        let headers = resp.headers().clone();
        let content_type = headers
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|h| h.to_str().ok())
            .map(String::from);
        let content = resp.text().await.map_err(Error::ResponseBody)?;

        if content_type == Some("application/json".to_string()) {
            let value = serde_json::from_str(&content).map_err(|e| Error::Decoding {
                status,
                content: content.to_string(),
                typename: std::any::type_name::<R>(),
                source: e,
            })?;
            Ok(Response {
                value,
                status,
                headers,
                content_type,
            })
        } else {
            Err(Error::ContentType {
                expected: vec!["application/json".to_string()],
                received: content_type,
            })
        }
    }

    pub async fn decode_id(resp: reqwest::Response) -> Result<i32> {
        let status = resp.status();
        let content_type = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|h| h.to_str().ok())
            .map(String::from);
        let content = resp.text().await.map_err(Error::ResponseBody)?;

        if content_type == Some("application/json".to_string()) {
            let fields: HashMap<&str, serde_json::Value> =
                serde_json::from_str(&content).map_err(|e| Error::Decoding {
                    status,
                    content: content.to_string(),
                    typename: "Object",
                    source: e,
                })?;
            serde_json::from_value(fields["id"].clone()).map_err(|e| Error::Decoding {
                status,
                content: fields["id"].to_string(),
                typename: "i32",
                source: e,
            })
        } else {
            Err(Error::ContentType {
                expected: vec!["application/json".to_string()],
                received: content_type,
            })
        }
    }

    pub async fn decode_bytes(resp: reqwest::Response) -> Result<Response<Bytes>> {
        let status = resp.status();
        let headers = resp.headers().clone();
        let content_type = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|h| h.to_str().ok())
            .map(String::from);
        let value = resp.bytes().await.map_err(Error::ResponseBody)?;

        Ok(Response {
            value,
            status,
            headers,
            content_type,
        })
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn ignore_content(resp: reqwest::Response) -> Result<Response<()>> {
        let status = resp.status();
        let headers = resp.headers().clone();
        let content_type = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|h| h.to_str().ok())
            .map(String::from);

        Ok(Response {
            value: (),
            status,
            headers,
            content_type,
        })
    }

    ////////////////////////////////////////////////////////////////////////////
    // pagination

    fn extract_params(url: &Url) -> Vec<(String, String)> {
        url.query_pairs()
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect::<Vec<_>>()
    }

    pub async fn previous_page<T>(
        &self,
        current: &Paginated<T>,
    ) -> Result<Option<Response<Paginated<T>>>>
    where
        T: for<'a> Deserialize<'a>,
    {
        if let Some(url) = current.raw_previous_url() {
            let params = Self::extract_params(url);
            let req = self.build_request(Method::GET, url.path(), &params, body::none)?;
            let resp = self.send_request(req).await?;
            Self::decode_json(resp).await.map(Some)
        } else {
            Ok(None)
        }
    }

    pub async fn next_page<T>(
        &self,
        current: &Paginated<T>,
    ) -> Result<Option<Response<Paginated<T>>>>
    where
        T: for<'a> Deserialize<'a>,
    {
        if let Some(url) = current.raw_next_url() {
            let params = Self::extract_params(url);
            let req = self.build_request(Method::GET, url.path(), &params, body::none)?;
            let resp = self.send_request(req).await?;
            Self::decode_json(resp).await.map(Some)
        } else {
            Ok(None)
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // services

    #[must_use]
    pub fn config(&self) -> &impl services::Config {
        self
    }

    #[must_use]
    pub fn correspondents(&self) -> &impl services::Correspondents {
        self
    }

    #[must_use]
    pub fn custom_fields(&self) -> &impl services::CustomFields {
        self
    }

    #[must_use]
    pub fn document_types(&self) -> &impl services::DocumentTypes {
        self
    }

    #[must_use]
    pub fn documents(&self) -> &impl services::Documents {
        self
    }

    #[must_use]
    pub fn groups(&self) -> &impl services::Groups {
        self
    }

    #[must_use]
    pub fn logs(&self) -> &impl services::Logs {
        self
    }

    #[must_use]
    pub fn mail_accounts(&self) -> &impl services::MailAccounts {
        self
    }

    #[must_use]
    pub fn mail_rules(&self) -> &impl services::MailRules {
        self
    }

    #[must_use]
    pub fn profile(&self) -> &impl services::Profile {
        self
    }

    #[must_use]
    pub fn saved_views(&self) -> &impl services::SavedViews {
        self
    }

    #[must_use]
    pub fn share_links(&self) -> &impl services::ShareLinks {
        self
    }

    #[must_use]
    pub fn storage_paths(&self) -> &impl services::StoragePaths {
        self
    }

    #[must_use]
    pub fn tags(&self) -> &impl services::Tags {
        self
    }

    #[must_use]
    pub fn tasks(&self) -> &impl services::Tasks {
        self
    }

    #[must_use]
    pub fn users(&self) -> &impl services::Users {
        self
    }

    #[must_use]
    pub fn workflows(&self) -> &impl services::Workflows {
        self
    }
}
