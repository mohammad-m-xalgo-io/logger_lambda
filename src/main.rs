// mod schema;
// mod services;
mod models;
mod utils;

// use crate::services::dynamodb_service::{DynamoDbService, query_dynamodb};
use crate::models::input_message::SqsEvent;
use crate::utils::logger;

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
// print!("{:?}", sqs_event);
// Create an instance of DynamoDbService
// let dynamodb_service = DynamoDbService::new("your_profile_name".to_string(), Region::UsEast1);
//
// // Query DynamoDB using the data from the SQS event
// let dynamodb_result = query_dynamodb(&dynamodb_service, sqs_event.table_name, sqs_event.partition_key, sqs_event.partition_value, sqs_event.sort_key).await?;
//
// // Log the DynamoDB result in JSON format
// log(json!(dynamodb_result));
//
// Ok(event.payload)
