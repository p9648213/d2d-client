use redis_pool::SingleRedisPool;

use super::config::EnvConfig;

pub fn create_pool(config: &EnvConfig) -> SingleRedisPool {
    let redis_client = redis::Client::open(config.redis_url.clone()).unwrap();
    redis_pool::RedisPool::from(redis_client)
}
