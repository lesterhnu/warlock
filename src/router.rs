use axum::routing::get;
use axum::Router;
use crate::handler;
pub fn router() -> Router {
    Router::new()
        .nest("/base", base_route())
        .nest("/user", user_route())
        .fallback(handler::not_found)
}


fn base_route() -> Router {
    Router::new()
        .route("/ping", get(handler::base::ping))
        .route("/test_config_error", get(handler::base::test_config_error))
}
fn user_route()->Router{
    Router::new()
        .route("/{id}",get(handler::user::get_user_by_id))
}