// mod schema;
// mod services;
mod models;
mod utils;
mod services;


// use crate::services::dynamodb_service::{DynamoDbService, query_dynamodb};
use crate::models::input_message::SqsEvent;
use crate::utils::logger;
use crate::services::dynamodb_service::DynamoDbService;
use lambda_runtime::{Error, LambdaEvent, Runtime};
use serde_json::Value;
use slog::{error, info};
use tower::service_fn;
use slog::{o};

// use std::env;

// pub fn get_environment() -> schema::Environment {
//     match env::var("ENVIRONMENT") {
//         Ok(val) => {
//             if val == "production" {
//                 schema::Environment::Production
//             } else if val == "staging" {
//                 schema::Environment::Staging
//             } else {
//                 panic!("Invalid environment: {}", val);
//             }
//         },
//         Err(_) => panic!("ENVIRONMENT variable not set"),
//     }
// }
//

#[tokio::main]
async fn main() -> Result<(), Error> {
    logger::init();
    let func = service_fn(func);
    Runtime::new(func).run().await?;
    Ok(())
}

// async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
//     match serde_json::from_value::<schema::GatewayCommandResponse>(event.payload.clone()) {
//         Ok(command_response) => {
//             dynamodb::query_dynamodb(command_response.command_id).await?;
//             Ok(event.payload)
//         }
//         Err(err) => Err(Error::from(format!("Deserialization error: {}", err))),
//     }
// }

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    // Parse the incoming SQS event
    let loggi = logger::LOGGER.get_logger();
    let payload: Value = event.payload.clone();
    if let Some(records) = payload.get("Records").and_then(Value::as_array) {
        for record in records {
            if let Some(body) = record.get("body").and_then(Value::as_str) {
                // Parse the body into an SqsEvent
                match serde_json::from_str::<SqsEvent>(body) {
                    Ok(sqs_event) => {
                        // Parsing was successful, continue processing
                        info!(loggi, "Message successfully parsed");
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

async fn process_record(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let loggi = logger::LOGGER.get_logger();

    // Parse the body into an SqsEvent
    let parsed_messages = message_parser(event.payload)?;

    // Query the DynamoDB table
    // let dynamodb_service = DynamoDbService::new(/* parameters */);
    // let query_result = dynamodb_service.query(/* parameters */).await?;
    //
    // // Log the result
    // info!(loggi, "Query result: {:?}", query_result);
    //
    // Ok(())
}

fn message_parser(payload: Value) -> Result<SqsEvent, Error> {
    let loggi = logger::LOGGER.get_logger();

    if let Some(records) = payload.get("Records").and_then(Value::as_array) {
        for record in records {
            if let Some(body) = record.get("body").and_then(Value::as_str) {
                // Parse the body into an SqsEvent
                match serde_json::from_str::<SqsEvent>(body) {
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
                        "body": "{\"entity_version\":1,\"command_id\":\"command1\",\"bet_offer_id\":\"offer1\",\"status\":\"PENDING\",\"status_reason_code\":\"reason1\",\"argument\":\"argument1\",\"message\":\"message1\",\"additional_entity_properties\":{\"keep_alive_expire_in_millis\":1000,\"non_combinable\":false,\"cash_out_payback\":1.23}}"
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