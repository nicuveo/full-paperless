#[derive(Debug)]
pub struct Response<R> {
    pub value: R,
    pub status: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub content_type: Option<String>,
}

impl<R> Response<R> {
    #[must_use]
    pub fn assign(self, item: &mut R) -> Response<()> {
        *item = self.value;
        Response {
            value: (),
            status: self.status,
            headers: self.headers,
            content_type: self.content_type,
        }
    }

    #[must_use]
    pub fn discard(self) -> Response<()> {
        Response {
            value: (),
            status: self.status,
            headers: self.headers,
            content_type: self.content_type,
        }
    }

    #[must_use]
    pub fn replace<T>(self, value: T) -> Response<T> {
        Response {
            value,
            status: self.status,
            headers: self.headers,
            content_type: self.content_type,
        }
    }
}
