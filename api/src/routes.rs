use crate::handlers::{collect::*, disperse::*};
use crate::state::AppState;
use axum::{routing::post, Router};
use std::sync::Arc;

pub fn collect_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/eth", post(collect_eth_handler))
        .route("/erc20", post(collect_erc20_handler))
        .with_state(state)
}

pub fn disperse_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/eth", post(disperse_eth_handler))
        .route("/erc20", post(disperse_erc20_handler))
        .with_state(state)
}
