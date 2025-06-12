#[derive(Debug)]
pub struct Response<R, E> {
    pub value: R,
    pub extra: E,
}

impl<R, E> Response<R, E> {
    #[must_use]
    pub fn assign(self, item: &mut R) -> Response<(), E> {
        *item = self.value;
        Response {
            value: (),
            extra: self.extra,
        }
    }

    #[must_use]
    pub fn discard(self) -> Response<(), E> {
        Response {
            value: (),
            extra: self.extra,
        }
    }

    #[must_use]
    pub fn replace<T>(self, value: T) -> Response<T, E> {
        Response {
            value,
            extra: self.extra,
        }
    }
}
