use crate::authentication::Auth;
use crate::error::{Error, Result};
use crate::response;
use crate::utils::Method;
use async_trait::async_trait;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::marker::Sync;
use std::time::{Duration, SystemTime};

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
}

pub type Response<R> = response::Response<R, Extra>;

pub struct Extra {
    pub method: Method,
    pub endpoint: String,
    pub status: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub duration: Duration,
    pub content_type: Option<String>,
}

#[async_trait]
impl super::Client for Client {
    type NetworkResponse = reqwest::Response;
    type Extra = Extra;

    async fn send<P, B>(
        &self,
        method: Method,
        endpoint: &str,
        params: &P,
        body: Option<&B>,
    ) -> Result<Response<reqwest::Response>>
    where
        B: Serialize + Sync,
        P: Serialize + Sync,
    {
        // build the request
        let uri = format!("{}{endpoint}", self.server_url);
        let mut request = self
            .inner
            .request(convert(method), &uri)
            .header(reqwest::header::ACCEPT, "application/json; version=9")
            .header(reqwest::header::AUTHORIZATION, self.auth.header_value())
            .query(params);
        if let Some(body) = body {
            request = request.json(body);
        }
        for (header_name, header_value) in &self.additional_headers {
            request = request.header(header_name, header_value);
        }
        let request = request.build().map_err(|e| Error::RequestBuild {
            method,
            endpoint: endpoint.to_string(),
            source: e.into(),
        })?;

        // send the request
        let start = SystemTime::now();
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
        let duration = start.elapsed().unwrap_or(Duration::from_secs(0));
        let status = resp.status();
        let headers = resp.headers().clone();
        let content_type = headers
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|h| h.to_str().ok())
            .map(String::from);

        match resp.error_for_status_ref() {
            Ok(_) => Ok(Response {
                value: resp,
                extra: Extra {
                    method,
                    endpoint: endpoint.to_string(),
                    status,
                    headers,
                    duration,
                    content_type,
                },
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

    async fn decode_json<R>(resp: Response<reqwest::Response>) -> Result<Response<R>>
    where
        R: for<'a> Deserialize<'a>,
    {
        // validate content-type
        if resp.extra.content_type != Some("application/json".to_string()) {
            return Err(Error::ContentType {
                method: resp.extra.method,
                endpoint: resp.extra.endpoint,
                expected: vec!["application/json".to_string()],
                received: resp.extra.content_type,
            });
        }

        // extract response data
        let content = resp
            .value
            .text()
            .await
            .map_err(|source| Error::ResponseBody {
                method: resp.extra.method,
                endpoint: resp.extra.endpoint.clone(),
                source: source.into(),
            })?;

        // decode the result
        let value = serde_json::from_str(&content).map_err(|source| Error::Deserializing {
            method: resp.extra.method,
            endpoint: resp.extra.endpoint.clone(),
            typename: std::any::type_name::<R>(),
            content,
            source,
        })?;
        Ok(Response {
            value,
            extra: resp.extra,
        })
    }

    async fn decode_bytes(resp: Response<reqwest::Response>) -> Result<Response<Bytes>> {
        let value = resp
            .value
            .bytes()
            .await
            .map_err(|source| Error::ResponseBody {
                method: resp.extra.method,
                endpoint: resp.extra.endpoint.clone(),
                source: source.into(),
            })?;
        Ok(Response {
            value,
            extra: resp.extra,
        })
    }
}

fn convert(method: Method) -> reqwest::Method {
    match method {
        Method::GET => reqwest::Method::GET,
        Method::PUT => reqwest::Method::PUT,
        Method::POST => reqwest::Method::POST,
        Method::PATCH => reqwest::Method::PATCH,
        Method::DELETE => reqwest::Method::DELETE,
    }
}
