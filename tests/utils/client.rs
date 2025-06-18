use paper_plane::auth::Auth;
use paper_plane::clients::{Client as _, reqwest::Client};
use paper_plane::error::Error;
use paper_plane::schema::api::users;
use paper_plane::schema::model::PermissionClass;
use paper_plane::services::Users;
use tokio;

use super::arbitrary;

const PAPERLESS_ADMIN_USERNAME: &str = "test";
const PAPERLESS_ADMIN_PASSWORD: &str = "test";

pub const PAPERLESS_URL: &str = "http://localhost:8101";

pub fn get() -> Client {
    let auth = Auth::Basic {
        username: PAPERLESS_ADMIN_USERNAME.to_string().into(),
        password: PAPERLESS_ADMIN_PASSWORD.to_string().into(),
    };
    Client::new(PAPERLESS_URL.to_string(), auth)
}

pub fn run_as_admin<F, R>(f: F) -> R
where
    F: AsyncFnOnce(Client) -> Result<R, Error>,
{
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { f(get()).await.unwrap_or_else(|e| panic!("{:?}", e)) })
}

pub fn run_as_user<F, R>(groups: Vec<i32>, permissions: Vec<PermissionClass>, f: F) -> R
where
    F: AsyncFnOnce(Client) -> Result<R, Error>,
{
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let client = get();
            let username = arbitrary::username();
            let password = "hardcoded";
            let _new_user = client
                .users()
                .create(
                    &users::create(username.clone())
                        .password(password.to_string())
                        .user_permissions(permissions)
                        .groups(groups),
                )
                .await
                .unwrap()
                .value;
            let auth = Auth::Basic {
                username: username.into(),
                password: password.to_string().into(),
            };
            let client = Client::new(PAPERLESS_URL.to_string(), auth);
            f(client).await.unwrap_or_else(|e| panic!("{:?}", e))
        })
}
