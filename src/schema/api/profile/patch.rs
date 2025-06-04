use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patch {
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[must_use]
pub fn patch() -> Patch {
    Patch::new()
}

impl Patch {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    #[must_use]
    pub fn password(mut self, value: String) -> Self {
        self.password = Some(value);
        self
    }

    #[must_use]
    pub fn first_name(mut self, value: String) -> Self {
        self.first_name = Some(value);
        self
    }

    #[must_use]
    pub fn last_name(mut self, value: String) -> Self {
        self.last_name = Some(value);
        self
    }
}
