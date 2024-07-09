use aws_config::{BehaviorVersion, defaults,profile};
use aws_sdk_s3::Client;
use aws_sdk_s3::error::{SdkError, UnknownVariantError};
use aws_sdk_s3::{Error as S3Error};
use bytes::Bytes;
use aws_smithy_types::byte_stream::ByteStream;
use crate::services::errors::MyError;
pub struct S3Client {
    client: Client,
}

pub trait S3ClientTrait {
    async fn get_object(&self, bucket: String, key: String) -> Result<Bytes, S3Error>;
    async fn put_object(&self, bucket: String, key: String, body:Bytes) -> Result<(), MyError>;
}

impl S3Client {
    pub async fn new(profile_name: String, region: String) -> Self {
        let config = defaults(BehaviorVersion::latest()).load().await;
        let client = Client::new(&config);

        Self { client }
    }
}

impl S3ClientTrait for S3Client {
    async fn get_object(&self, bucket: String, key: String) -> Result<Bytes, S3Error> {
        let resp = self.client.get_object().bucket(bucket).key(key).send().await?;
        let body = match resp.body.collect().await {
            Ok(body) => body.into_bytes(),
            Err(e) => {
                // eprintln!("Error collecting body: {:?}", e);
                return Ok(Bytes::new());
            }
        };
        Ok(body)
    }

    async fn put_object(&self, bucket: String, key: String, body: Bytes) -> Result<(), MyError> {
        let body: ByteStream = ByteStream::from(body);
        // self.client.put_object().bucket(bucket).key(key).body(body).send().await?;
        match self.client.put_object().bucket(bucket).key(key).body(body).send().await {
            Ok(_) => Ok(()),
            Err(err) => Err(MyError{message: format!("Failed to put object: {}", err),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[tokio::test]
    async fn test_put_object() {
        let s3_client = S3Client::new("xalgo_kambi_adapter".to_string(), "eu-west-1".to_string()).await;
        s3_client.put_object("development-tzeract-platf-baseresourcesinfradevelo-4z5uolomql2t".to_string(), "test-day/test/key_2".to_string(), Bytes::from("test-body")).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_object() {
        let s3_client = S3Client::new("xalgo_kambi_adapter".to_string(), "eu-west-1".to_string()).await;
        let body = s3_client.get_object("development-tzeract-platf-baseresourcesinfradevelo-4z5uolomql2t".to_string(), "test-key".to_string()).await.unwrap();
        assert_eq!(body, Bytes::from("test-body"));
    }
}