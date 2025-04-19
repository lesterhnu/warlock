use warlock::CONFIG;
use warlock::boot;
use tokio::signal;
#[tokio::main]
async fn main() {
    let _guard = boot::init().await.unwrap();
    let app = warlock::router::router();
    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        CONFIG.get_string("app.host").unwrap_or("localhost".to_string()),
        CONFIG.get_int("app.port").unwrap_or(8080)
    ))
    .await
    .unwrap();
    tracing::debug!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app)
    .with_graceful_shutdown(shut_down())
    .await.unwrap();
}

async fn shut_down() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Ctrl+C received, shutting down");
        },
        _ = terminate => {},
    }
}