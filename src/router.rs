use axum::routing::{get, post};
use axum::Router;
use crate::handler;
pub fn router() -> Router {
    Router::new()
        .nest("/base", base_route())
        .nest("/user", user_route())
        .nest("/banner", banner_route())
        .fallback(handler::not_found)
}


fn base_route() -> Router {
    Router::new()
        .route("/ping", get(handler::base::ping))
        .route("/test_config_error", get(handler::base::test_config_error))
        .route("/upload", post(handler::base::upload_file))
        .route("/test_request", get(handler::base::test_reqwest))
}
fn banner_route()->Router{
    Router::new()
        .route("/get_banners",get(handler::base::get_banner))
}
fn user_route()->Router{
    Router::new()
        .route("/{id}",get(handler::user::get_user_by_id))
        .route("/create_user", post(handler::user::create_user))
        .route("/login", post(handler::user::login))
}