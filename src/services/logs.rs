use crate::clients::Client;
use crate::error::Result;
use crate::response::Response;
use crate::utils::{Method, body, params};
use async_trait::async_trait;

#[async_trait]
pub trait Logs<E = ()> {
    async fn list(&self) -> Result<Response<Vec<String>, E>>;
    async fn retrieve(&self, log_type: &str) -> Result<Response<Vec<String>, E>>;
}

#[async_trait]
impl<C: Client> Logs<C::Extra> for C {
    async fn list(&self) -> Result<Response<Vec<String>, C::Extra>> {
        let path = "/api/logs/";
        self.request_json(Method::GET, path, params::NONE, body::NONE)
            .await
    }

    async fn retrieve(&self, log_type: &str) -> Result<Response<Vec<String>, C::Extra>> {
        let path = format!("/api/logs/{log_type}/");
        self.request_json(Method::GET, &path, params::NONE, body::NONE)
            .await
    }
}
