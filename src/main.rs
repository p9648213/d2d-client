use clap::Parser;
use d2d_client::{
    config::{self},
    postgres,
    router::create_router,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let config = config::EnvConfig::parse();

    tracing::info!("Redis pool created");

    let pg_pool = postgres::create_pool(&config);

    tracing::info!("Postgres pool created");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &config.port))
        .await
        .unwrap();

    let app = create_router(pg_pool, config);

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.await).await.unwrap();
}
