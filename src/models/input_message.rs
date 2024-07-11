use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::option::Option;

#[derive(Debug, Serialize, Deserialize)]
pub enum StatusBase {
    PENDING,
    SUCCESS,
    FAILURE,
    REJECTED,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalProp {
    pub keep_alive_expire_in_millis: Option<i32>,
    pub non_combinable: Option<bool>,
    pub cash_out_payback: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SqsEvent {
    pub entity_version: i32,
    pub command_id: String,
    // #[serde(rename = "entityId")]
    pub bet_offer_id: String,
    pub status: StatusBase,
    pub status_reason_code: Option<String>,
    pub argument: Option<String>,
    pub message: Option<String>,
    pub additional_entity_properties: Option<AdditionalProp>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub destination_type: String,
    pub destination_arn: String,
    pub destination_name: String,
    pub processor_type: String,
    pub context : String,
    pub context_params: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DynamoDBEvent {
    #[serde(alias = "eventID")]
    event_id: String,
    #[serde(alias = "eventName")]
    pub event_name: String,
    #[serde(alias = "eventVersion")]
    event_version: String,
    #[serde(alias = "eventSource")]
    pub event_source: String,
    #[serde(alias = "awsRegion")]
    aws_region: String,
    pub dynamodb: Value,
    #[serde(alias = "eventSourceARN")]
    event_source_arn: String,
}
// pub enum Environment {
//     Production,
//     Staging,
//     // Add more as needed
// }
