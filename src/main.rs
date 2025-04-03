use warlock::CONFIG;
use warlock::boot;

#[tokio::main]
async fn main() {
    let _guard = boot::init().unwrap();
    let app = warlock::router::router();
    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        CONFIG.get_string("app.host").unwrap_or("localhost".to_string()),
        CONFIG.get_int("app.port").unwrap_or(8080)
    ))
    .await
    .unwrap();
    tracing::debug!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
