#[derive(Debug)]
pub enum Error {
    RequestBuild(reqwest::Error),
    RequestSend(reqwest::Error),
    ContentType {
        expected: Vec<String>,
        received: Option<String>,
    },
    ResponseBody(reqwest::Error),
    Decoding {
        status: reqwest::StatusCode,
        content: String,
        typename: &'static str,
        source: serde_json::Error,
    },
    Application {
        status: reqwest::StatusCode,
        content: serde_json::Value,
    },
    WorkflowItem {
        id: i32,
    },
}

pub type Result<R> = std::result::Result<R, Error>;
