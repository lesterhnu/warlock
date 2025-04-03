use axum::routing::get;
use axum::Router;
use crate::app::handler;
pub fn router() -> Router {
    Router::new()
        .nest("/base", base_route())
        .fallback(handler::not_found)
}


fn base_route() -> Router {
    Router::new()
        .route("/ping", get(handler::base::ping))
        .route("/test_config_error", get(handler::base::test_config_error))
}