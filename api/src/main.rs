use api::config::AppConfig;
use api::routes::{collect_routes, disperse_routes};
use api::state::AppState;
use axum::Router;
use core::result::Result;
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();
    let config = AppConfig::load()?;
    let port = config.port;
    let state: Arc<AppState> = AppState::init(config).await?;

    let collect_routes = collect_routes(state.clone());
    let disperse_routes = disperse_routes(state);

    let app = Router::new()
        .nest("/collect", collect_routes)
        .nest("/disperse", disperse_routes);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:".to_string() + &port.to_string())
        .await
        .unwrap();
    info!("Server is running on 127.0.0.1:{}", port);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
