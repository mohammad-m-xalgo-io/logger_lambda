use std::env;
use redis::{Client, AsyncCommands};
use redis::aio::MultiplexedConnection;
struct RedisClient {
    client: MultiplexedConnection,
}
trait RedisClientTrait {
    async fn get_value(&self, key: u64) -> Result<String, redis::RedisError>;
    async fn set_value(&self, key: u64, value: String) -> Result<(), redis::RedisError>;
}
impl RedisClient {
    pub async fn new(connection_string: Option<&str>) -> Self {
        let client = Client::open(connection_string.unwrap_or("redis://127.0.0.1/")).unwrap().get_multiplexed_tokio_connection().await.unwrap();
        Self { client }
    }
}
impl RedisClientTrait for RedisClient {
    async fn get_value(&self, key: u64) -> Result<String, redis::RedisError> {
        let mut con = self.client.clone();
        let value: String = con.get(key).await?;
        Ok(value)
    }
    async fn set_value(&self, key: u64, value: String) -> Result<(), redis::RedisError> {
        let mut con = self.client.clone();
        con.set(key, value).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use redis::Value;
    use redis::ErrorKind;
    use redis::RedisError;

    #[tokio::test]
    async fn test_set_value() {
        let client = RedisClient::new(Some("redis://127.0.0.1/")).await;

        client.set_value(1, "test_value".to_string()).await.unwrap();

        let value = client.get_value(1).await.unwrap();
        assert_eq!(value, "test_value".to_string());
    }
}