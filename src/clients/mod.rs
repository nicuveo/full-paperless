use async_trait::async_trait;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::marker::Sync;

use crate::error::Result;
use crate::response::Response;
use crate::schema::model::Paginated;
use crate::services;
use crate::utils::{Method, body, extract_params};

////////////////////////////////////////////////////////////////////////////////
// Public implementations

#[cfg(feature = "reqwest")]
pub mod reqwest;

////////////////////////////////////////////////////////////////////////////////
// Public trait

#[async_trait]
pub trait Client: Sized + Sync {
    // network

    type Extra;

    async fn request_json<P, B, R>(
        &self,
        method: Method,
        endpoint: &str,
        params: &P,
        body: Option<&B>,
    ) -> Result<Response<R, Self::Extra>>
    where
        P: Serialize + Sync,
        B: Serialize + Sync,
        R: for<'a> Deserialize<'a>;

    async fn request_bytes<P, B>(
        &self,
        method: Method,
        endpoint: &str,
        params: &P,
        body: Option<&B>,
    ) -> Result<Response<Bytes, Self::Extra>>
    where
        P: Serialize + Sync,
        B: Serialize + Sync;

    async fn request_unit<P, B>(
        &self,
        method: Method,
        endpoint: &str,
        params: &P,
        body: Option<&B>,
    ) -> Result<Response<(), Self::Extra>>
    where
        P: Serialize + Sync,
        B: Serialize + Sync;

    // pagination

    async fn previous_page<T>(
        &self,
        current: &Paginated<T>,
    ) -> Result<Option<Response<Paginated<T>, Self::Extra>>>
    where
        T: for<'a> Deserialize<'a> + Sync,
    {
        if let Some(url) = current.raw_previous_url() {
            self.request_json(Method::GET, url.path(), &extract_params(url), body::NONE)
                .await
                .map(Some)
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
            self.request_json(Method::GET, url.path(), &extract_params(url), body::NONE)
                .await
                .map(Some)
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
