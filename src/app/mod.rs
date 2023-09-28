mod jwt;
mod router;
mod schema;

use axum::{extract::DefaultBodyLimit, routing::get, Router, Server};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::OnceCell;

pub static APP_STATE: OnceCell<Arc<AppState>> = OnceCell::const_new();
pub struct AppState {
    pub oss_client: homeland_backend::oss_client::OssClient,
    pub schema: schema::AppSchema,
}

impl AppState {
    async fn init() -> anyhow::Result<Arc<AppState>> {
        Ok(Arc::new(Self {
            oss_client: homeland_backend::oss_client::OssClient::from_env()?,
            schema: schema::build_schema(),
        }))
    }
}

pub async fn init_app_state() -> anyhow::Result<Arc<AppState>> {
    let app_state = APP_STATE.get_or_try_init(|| async move { AppState::init().await }).await?;
    Ok(app_state.clone())
}

pub async fn listen(addr: SocketAddr) -> anyhow::Result<()> {
    let state = crate::app::AppState::init().await?;
    let app = Router::new()
        .route(
            "/",
            get(|claims: Option<jwt::Claims>| async move {
                let mut text = "Welcome".to_string();
                if let Some(claims) = claims {
                    text.push_str(&format!(", {:?}", claims.sub));
                }
                text
            }),
        )
        .nest("/", router::compose())
        .with_state(state)
        .layer(DefaultBodyLimit::max(2 * 1024 * 1024));

    log::debug!("GraphiQL IDE: {}/graphql", &addr);
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
