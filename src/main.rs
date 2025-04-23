use axum_session_redis_bb8_pool::SessionRedisPool;
use bb8_redis::bb8;
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

    // let redis_client = redis::Client::open(config.redis_url.clone())
    //     .expect("Error while trying to open the redis connection");

    let manager = bb8_redis::RedisConnectionManager::new(config.redis_url.clone()).unwrap();

    let redis_pool = bb8::Pool::builder().build(manager).await.unwrap();

    tracing::info!("Redis pool created");

    let pg_pool = postgres::create_pool(&config);

    tracing::info!("Postgres pool created");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &config.port))
        .await
        .unwrap();
    

    let app = create_router(pg_pool, SessionRedisPool::try_from(redis_pool).unwrap(), config);

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.await).await.unwrap();
}
