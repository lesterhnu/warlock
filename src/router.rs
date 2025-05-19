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
        .route("/get_sign", get(handler::base::get_oss_sign))
}
fn banner_route()->Router{
    Router::new()
        .route("/get_banners",get(handler::banner::get_banner))
        .route("/create+banner", post(handler::banner::create_banner))
        .route("/delete", get(handler::banner::delete_banner))
        .route("/batch_delete", post(handler::banner::batch_delete_banner))
        
}
fn user_route()->Router{
    Router::new()
        .route("/{id}",get(handler::user::get_user_by_id))
        .route("/create_user", post(handler::user::create_user))
        .route("/login", post(handler::user::login))
}