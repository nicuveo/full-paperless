use reqwest::RequestBuilder;
use serde::Serialize;

pub fn none(builder: RequestBuilder) -> RequestBuilder {
    builder
}

pub fn json<B: Serialize>(body: &B) -> impl (FnOnce(RequestBuilder) -> RequestBuilder) {
    |builder| builder.json(body)
}
