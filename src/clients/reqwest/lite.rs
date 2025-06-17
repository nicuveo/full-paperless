use async_trait::async_trait;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::marker::Sync;

use super::translate_method;
use crate::auth::Auth;
use crate::clients::Client as ClientTrait;
use crate::error::{Error, Result};
use crate::response;
use crate::utils::Method;

////////////////////////////////////////////////////////////////////////////////
// Public types

#[derive(Debug, Clone)]
pub struct Client {
    inner: reqwest::Client,
    server_url: String,
    auth: Auth,
}

pub struct InternalResponse {
    pub inner: reqwest::Response,
    method: Method,
    endpoint: String,
    content_type: Option<String>,
}

pub type Response<R> = response::Response<R, ()>;

////////////////////////////////////////////////////////////////////////////////
// Public implementation

impl Client {
    #[must_use]
    pub fn new(server_url: String, auth: Auth) -> Self {
        Self {
            inner: reqwest::Client::new(),
            server_url,
            auth,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Traits

#[async_trait]
impl ClientTrait for Client {
    type IntermediateResponse = InternalResponse;
    type Extra = ();

    async fn send<P, B>(
        &self,
        method: Method,
        endpoint: &str,
        params: &P,
        body: Option<&B>,
    ) -> Result<Self::IntermediateResponse>
    where
        B: Serialize + Sync,
        P: Serialize + Sync,
    {
        // build the request
        let uri = format!("{}{endpoint}", self.server_url);
        let mut request = self
            .inner
            .request(translate_method(method), &uri)
            .header(reqwest::header::ACCEPT, "application/json; version=9")
            .header(reqwest::header::AUTHORIZATION, self.auth.header_value())
            .query(params);
        if let Some(body) = body {
            request = request.json(body);
        }
        let request = request.build().map_err(|e| Error::RequestBuild {
            method,
            endpoint: endpoint.to_string(),
            source: e.into(),
        })?;

        // send the request
        let resp = self
            .inner
            .execute(request)
            .await
            .map_err(|source| Error::RequestSend {
                method,
                endpoint: endpoint.to_string(),
                source: source.into(),
            })?;

        // validate request status
        let status = resp.status();
        let content_type = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|h| h.to_str().ok())
            .map(String::from);

        match resp.error_for_status_ref() {
            Ok(_) => Ok(InternalResponse {
                inner: resp,
                method,
                endpoint: endpoint.to_string(),
                content_type,
            }),
            Err(source) => {
                let content = match resp.text().await {
                    Err(_) => serde_json::Value::String("<failed to retrieve content>".to_string()),
                    Ok(content) => {
                        serde_json::from_str(&content).unwrap_or(serde_json::Value::String(content))
                    }
                };
                Err(Error::Server {
                    method,
                    endpoint: endpoint.to_string(),
                    status: format!("{status}"),
                    content,
                    source: source.into(),
                })
            }
        }
    }

    async fn decode_json<R>(resp: Self::IntermediateResponse) -> Result<Response<R>>
    where
        R: for<'a> Deserialize<'a>,
    {
        // validate content-type
        if resp.content_type != Some("application/json".to_string()) {
            return Err(Error::ContentType {
                method: resp.method,
                endpoint: resp.endpoint,
                expected: vec!["application/json".to_string()],
                received: resp.content_type,
            });
        }

        // extract response data
        let content = resp
            .inner
            .text()
            .await
            .map_err(|source| Error::ResponseBody {
                method: resp.method,
                endpoint: resp.endpoint.clone(),
                source: source.into(),
            })?;

        // decode the result
        let value = serde_json::from_str(&content).map_err(|source| Error::Deserializing {
            method: resp.method,
            endpoint: resp.endpoint.clone(),
            typename: std::any::type_name::<R>(),
            content,
            source,
        })?;
        Ok(Response { value, extra: () })
    }

    async fn decode_bytes(resp: Self::IntermediateResponse) -> Result<Response<Bytes>> {
        let value = resp
            .inner
            .bytes()
            .await
            .map_err(|source| Error::ResponseBody {
                method: resp.method,
                endpoint: resp.endpoint,
                source: source.into(),
            })?;
        Ok(Response { value, extra: () })
    }

    fn ignore_content(_resp: Self::IntermediateResponse) -> Result<Response<()>> {
        Ok(Response {
            value: (),
            extra: (),
        })
    }
}
