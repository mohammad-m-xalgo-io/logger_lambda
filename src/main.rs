mod models;
mod processors;
mod services;
mod utils;

use bytes::Bytes;
use std::fmt::format;
// use crate::services::dynamodb_service::{DynamoDbService, query_dynamodb};
use crate::models::input_message::{DynamoDBEvent, Message};
use crate::services::message_factory::PayloadFactory;
use crate::services::s3::s3_client::S3ClientTrait;
use crate::utils::logger;
use chrono::{Datelike, Utc};
use lambda_runtime::{Error, LambdaEvent, Runtime};
use serde_json::{json, Value};
use services::s3::s3_client::S3Client;
use slog::o;
use slog::{error, info};
use tower::service_fn;
use serde_dynamo::{AttributeValue, Item};


#[tokio::main]
async fn main() -> Result<(), Error> {
    logger::init();
    let func = service_fn(func);
    Runtime::new(func).run().await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let loggi = logger::LOGGER.get_logger();
    let payload: Value = event.payload.clone();
    if let Some(records) = payload.as_array() {
        for record in records {
            let record_str = record.to_string();
            match serde_json::from_str::<DynamoDBEvent>(&record_str) {
                Ok(dynamodb_event) => {
                    match dynamodb_event {
                        DynamoDBEvent {
                            event_name,
                            event_source,
                            dynamodb,
                            ..
                        } if event_name == "REMOVE" && event_source == "aws:dynamodb" => {

                            let item: Item = serde_json::from_str(dynamodb.get("Keys").expect("WF").to_string().as_str())
                                .expect("expected to deserialize DynamoDB JSON format");

                            let mik = item.get("event_xid").and_then(|v| match v {
                                AttributeValue::S(s) => s.parse::<String>().ok(),
                                _ => None,
                            });
                            todo!("mik:");
                        }
                        _ => {
                            // Handle all other cases
                        }
                    }
                }
                Err(err) => {
                    // Parsing failed, log an error message
                    error!(loggi, "Failed to parse DynamoDB event: {}", err);
                    return Err(Error::from(err));
                }
            }
        }
    }
    error!(loggi, "No valid records found in payload"; o!("payload" => payload.to_string()));
    Err(Error::from("No valid records found in payload"))
}

fn message_parser(payload: Value) -> Result<Message, Error> {
    let loggi = logger::LOGGER.get_logger();

    if let Some(records) = payload.get("Records").and_then(Value::as_array) {
        for record in records {
            if let Some(body) = record.get("body").and_then(Value::as_str) {
                // Parse the body into an SqsEvent
                match serde_json::from_str::<Message>(body) {
                    Ok(sqs_event) => {
                        // Parsing was successful, continue processing
                        info!(loggi, "Message successfully parsed");
                        return Ok(sqs_event);
                    }
                    Err(err) => {
                        // Parsing failed, log an error message
                        error!(loggi, "Failed to parse SQS event: {}", err);
                        return Err(Error::from(err));
                    }
                }
            }
        }
    }
    Err(Error::from("No valid records found in payload"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::LambdaEvent;
    use serde_json::json;
    use std::fs;

    #[tokio::test]
    async fn test_func() {
        // Create a mock LambdaEvent
        let file_content = std::fs::read_to_string("src/test/dynamodb_event.json").unwrap();
        let payload = serde_json::from_str(&file_content).unwrap();
        let mock_event = LambdaEvent {
            payload,
            context: lambda_runtime::Context::default(),
        };

        // Call the function with the mock event
        let result = func(mock_event).await;

        // Assert that the function returned Ok
        assert!(result.is_ok());

        // Assert that the returned payload is the same as the input
    }
}
