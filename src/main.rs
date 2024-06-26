// mod schema;
// mod services;
mod models;
mod utils;
mod services;


use bytes::Bytes;
// use crate::services::dynamodb_service::{DynamoDbService, query_dynamodb};
use crate::models::input_message::Message;
use crate::utils::logger;
use lambda_runtime::{Error, LambdaEvent, Runtime};
use serde_json::Value;
use slog::{error, info};
use tower::service_fn;
use slog::{o};
use services::s3_client::S3Client;
use crate::services::s3_client::S3ClientTrait;

#[tokio::main]
async fn main() -> Result<(), Error> {
    logger::init();
    let func = service_fn(func);
    Runtime::new(func).run().await?;
    Ok(())
}


async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    // Parse the incoming SQS event
    let loggi = logger::LOGGER.get_logger();
    let payload: Value = event.payload.clone();
    if let Some(records) = payload.get("Records").and_then(Value::as_array) {
        for record in records {
            if let Some(body) = record.get("body").and_then(Value::as_str) {
                // Parse the body into an SqsEvent
                match serde_json::from_str::<Message>(body) {
                    Ok(sqs_event) => {
                        // Parsing was successful, continue processing
                        info!(loggi, "Message successfully parsed");
                        let s3_client = S3Client::new("xalgo_kambi_adapter".to_string(), "eu-west-1".to_string()).await;
                        s3_client.put_object(sqs_event.destination_name, sqs_event.destination_arn, Bytes::from(sqs_event.context)).await.unwrap();
                        return Ok(event.payload);
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

    #[tokio::test]
    async fn test_func() {
        // Create a mock LambdaEvent
        let mock_event = LambdaEvent {
            payload: json!({
                "Records": [
                    {
                        "body": "{\"destination_arn\":\"MyTopic\",\"destination_name\":\"development-tzeract-platf-baseresourcesinfradevelo-4z5uolomql2t\",\"context\":\"context1\",\"context_params\":\"context_params1\"}"
                    }
                ]
            }),
            context: lambda_runtime::Context::default(),
        };

        // Call the function with the mock event
        let result = func(mock_event).await;

        // Assert that the function returned Ok
        assert!(result.is_ok());

        // Assert that the returned payload is the same as the input
    }
}