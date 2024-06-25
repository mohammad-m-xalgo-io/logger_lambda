use std::env;
use redis::{Client, Commands};

struct RedisClient {
    client: Client,
}
trait RedisClientTrait {
    async fn get_value(&self, key: u64) -> Result<String, redis::RedisError>;
    async fn set_value(&self, key: u64, value: String) -> Result<(), redis::RedisError>;
}
impl RedisClient {
    pub fn new(connection_string: Option<&str>) -> Self {
        let client = Client::open(connection_string.unwrap_or("redis://127.0.0.1/")).unwrap();
        Self { client }
    }
}
impl RedisClientTrait for RedisClient {
    async fn get_value(&self, key: u64) -> Result<String, redis::RedisError> {
        let mut con = self.client.get_connection()?;
        let value: String = con.get(key)?;
        Ok(value)
    }
    async fn set_value(&self, key: u64, value: String) -> Result<(), redis::RedisError> {
        let mut con = self.client.get_connection()?;
        con.set(key, value)?;
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
        let client = RedisClient::new(Some("redis://127.0.0.1/"));

        client.set_value(1, "test_value".to_string()).await.unwrap();

        let value = client.get_value(1).await.unwrap();
        assert_eq!(value, "test_value".to_string());
    }
}