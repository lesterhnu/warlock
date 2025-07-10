use axum::routing::{get, post};
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::cors::{Any,  CorsLayer};
use crate::handler;
pub fn router() -> Router {
    Router::new()
        .nest("/base", base_route())
        .nest("/user", user_route())
        .nest("/book", book_route())
        .nest("/admin", admin_route())
        .nest_service("/favicon.ico", ServeFile::new("static/favicon.ico"))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(handler::not_found)
        .layer(CorsLayer::new().allow_origin(Any))
}

fn admin_route()->Router{
    Router::new()
        .route("/login",get(handler::admin::login_ui))
        .route("/upload",get(handler::admin::upload_ui))   

}

fn base_route() -> Router {
    Router::new()
        .route("/ping", get(handler::base::ping))
        .route("/test_config_error", get(handler::base::test_config_error))
        .route("/upload", post(handler::base::upload_file))
        .route("/test_request", get(handler::base::test_reqwest))
        .route("/get_sign", get(handler::base::get_oss_sign))
        .route("/iframe_test", get(handler::base::iframe_test_ui))
}
fn book_route()->Router{
    Router::new()
        .route("/get_books",get(handler::book::get_book))
        // .route("/create+banner", post(handler::book::create_banner))
        // .route("/delete", get(handler::book::delete_banner))
        // .route("/batch_delete", post(handler::book::batch_delete_banner))
        
}
fn user_route()->Router{
    Router::new()
        .route("/{id}",get(handler::user::get_user_by_id))
        .route("/create_user", post(handler::user::create_user))
        .route("/login", post(handler::user::login))
}