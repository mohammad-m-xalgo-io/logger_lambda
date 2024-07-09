use crate::services::formatters::Formatter;
use crate::processors::outcome_processor::OutcomeProcessor;
use crate::processors::bet_offer_processor::BetOfferProcessor;

use crate::services::s3::s3_client::S3Client;

pub struct PayloadFactory;
impl PayloadFactory {
    pub fn create_formater(processor_type: String) -> Box<dyn Formatter> {
        match processor_type.as_str() {
            "1" => Box::new(OutcomeProcessor::new()),
            "2" => Box::new(BetOfferProcessor::new()),
            _ => panic!("Unknown processor type"),
        }
    }
}


pub struct ClientFactory;
impl ClientFactory {
    pub fn create_client(processor_type: String) -> Box<dyn Formatter> {
        match processor_type.as_str() {
            "1" => Box::new(S3Client::new("xalgo_kambi_adapter".to_string(), "eu-west-1".to_string())),
            _ => panic!("Unknown processor type"),
        }
    }
}