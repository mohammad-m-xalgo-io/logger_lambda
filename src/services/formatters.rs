pub trait Formatter {
    fn formater(&self) -> String;
}


pub trait S3PayloadFormatter {
    fn s3_format_payload(&self, payload: String) -> String;
}

pub trait DynamoPayloadFormatter {
    fn dynamo_format_payload(&self, payload: String) -> String;
}