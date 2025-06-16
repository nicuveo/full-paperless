use async_trait::async_trait;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::marker::Sync;

use crate::error::Result;
use crate::response::Response;
use crate::schema::model::Paginated;
use crate::services;
use crate::utils::{Method, body, extract_params};

#[cfg(feature = "reqwest")]
pub mod reqwest;

#[async_trait]
pub trait Client: Sync + Send {
    // network

    type NetworkResponse: Send + Sized;
    type Extra: Send + Sized;

    async fn send<P, B>(
        &self,
        method: Method,
        endpoint: &str,
        params: &P,
        body: Option<&B>,
    ) -> Result<Response<Self::NetworkResponse, Self::Extra>>
    where
        P: Serialize + Sync,
        B: Serialize + Sync;

    async fn decode_json<R>(
        response: Response<Self::NetworkResponse, Self::Extra>,
    ) -> Result<Response<R, Self::Extra>>
    where
        R: for<'a> Deserialize<'a>;

    async fn decode_bytes(
        response: Response<Self::NetworkResponse, Self::Extra>,
    ) -> Result<Response<Bytes, Self::Extra>>;

    fn ignore_content(
        response: Response<Self::NetworkResponse, Self::Extra>,
    ) -> Result<Response<(), Self::Extra>> {
        Ok(response.replace(()))
    }

    async fn decode_id(response: Response<Self::NetworkResponse, Self::Extra>) -> Result<i32> {
        let mut fields = Self::decode_json::<HashMap<String, serde_json::Value>>(response).await?;
        Ok(serde_json::from_value(fields.value.remove("id").unwrap()).unwrap())
    }

    // pagination

    async fn previous_page<T>(
        &self,
        current: &Paginated<T>,
    ) -> Result<Option<Response<Paginated<T>, Self::Extra>>>
    where
        T: for<'a> Deserialize<'a> + Sync,
    {
        if let Some(url) = current.raw_previous_url() {
            let resp = self
                .send(Method::GET, url.path(), &extract_params(url), body::NONE)
                .await?;
            Self::decode_json(resp).await.map(Some)
        } else {
            Ok(None)
        }
    }

    async fn next_page<T>(
        &self,
        current: &Paginated<T>,
    ) -> Result<Option<Response<Paginated<T>, Self::Extra>>>
    where
        T: for<'a> Deserialize<'a> + Sync,
    {
        if let Some(url) = current.raw_next_url() {
            let resp = self
                .send(Method::GET, url.path(), &extract_params(url), body::NONE)
                .await?;
            Self::decode_json(resp).await.map(Some)
        } else {
            Ok(None)
        }
    }

    // services

    fn config(&self) -> &impl services::Config<Self::Extra>
    where
        Self: Sized,
    {
        self
    }

    fn correspondents(&self) -> &impl services::Correspondents<Self::Extra>
    where
        Self: Sized,
    {
        self
    }

    fn custom_fields(&self) -> &impl services::CustomFields<Self::Extra>
    where
        Self: Sized,
    {
        self
    }

    fn document_types(&self) -> &impl services::DocumentTypes<Self::Extra>
    where
        Self: Sized,
    {
        self
    }

    fn documents(&self) -> &impl services::Documents<Self::Extra>
    where
        Self: Sized,
    {
        self
    }

    fn groups(&self) -> &impl services::Groups<Self::Extra>
    where
        Self: Sized,
    {
        self
    }

    fn logs(&self) -> &impl services::Logs<Self::Extra>
    where
        Self: Sized,
    {
        self
    }

    fn mail_accounts(&self) -> &impl services::MailAccounts<Self::Extra>
    where
        Self: Sized,
    {
        self
    }

    fn mail_rules(&self) -> &impl services::MailRules<Self::Extra>
    where
        Self: Sized,
    {
        self
    }

    fn profile(&self) -> &impl services::Profile<Self::Extra>
    where
        Self: Sized,
    {
        self
    }

    fn saved_views(&self) -> &impl services::SavedViews<Self::Extra>
    where
        Self: Sized,
    {
        self
    }

    fn share_links(&self) -> &impl services::ShareLinks<Self::Extra>
    where
        Self: Sized,
    {
        self
    }

    fn storage_paths(&self) -> &impl services::StoragePaths<Self::Extra>
    where
        Self: Sized,
    {
        self
    }

    fn tags(&self) -> &impl services::Tags<Self::Extra>
    where
        Self: Sized,
    {
        self
    }

    fn tasks(&self) -> &impl services::Tasks<Self::Extra>
    where
        Self: Sized,
    {
        self
    }

    fn users(&self) -> &impl services::Users<Self::Extra>
    where
        Self: Sized,
    {
        self
    }

    fn workflows(&self) -> &impl services::Workflows<Self::Extra>
    where
        Self: Sized,
    {
        self
    }
}
