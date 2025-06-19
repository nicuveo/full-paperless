// use std::backtrace::Backtrace;
use thiserror::Error;

use crate::utils::Method;

#[derive(Error, Debug)]
pub enum Error {
    #[error("[{:?} {}] failed to build a request", .method, .endpoint)]
    RequestBuild {
        method: Method,
        endpoint: String,
        // #[backtrace]
        source: anyhow::Error,
    },
    #[error("[{:?} {}] failed to send the request", .method, .endpoint)]
    RequestSend {
        method: Method,
        endpoint: String,
        // #[backtrace]
        source: anyhow::Error,
    },
    #[error("[{:?} {}] server error\nstatus: {}\nmessage: {}", .method, .endpoint, .status, .content)]
    Server {
        method: Method,
        endpoint: String,
        status: String,
        content: serde_json::Value,
        // #[backtrace]
        source: anyhow::Error,
    },
    #[error("[{:?} {}] incorrect content type\nexpecting: {}\nreceived: {}", .method, .endpoint, .expected.join(", "), .received.as_deref().unwrap_or("<none>"))]
    ContentType {
        method: Method,
        endpoint: String,
        expected: Vec<String>,
        received: Option<String>,
        // backtrace: Backtrace,
    },
    #[error("[{:?} {}] failed to retrieve response body", .method, .endpoint)]
    ResponseBody {
        method: Method,
        endpoint: String,
        // #[backtrace]
        source: anyhow::Error,
    },
    #[error("[{:?} {}] failed to decode response body\nexpected type: {}\nraw string: {:?}", .method, .endpoint, .typename, .content)]
    Deserializing {
        method: Method,
        endpoint: String,
        typename: &'static str,
        content: String,
        // #[backtrace]
        source: serde_json::Error,
    },
    #[error("internal server error")]
    Internal {
        source: anyhow::Error,
        // backtrace: Backtrace,
    },
}

pub type Result<R> = std::result::Result<R, Error>;
