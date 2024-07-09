use crate::services::formatters::{S3PayloadFormatter, Formatter};
pub struct BetOfferProcessor {
    pub payload: String,
}

impl BetOfferProcessor {
    pub fn new() -> BetOfferProcessor {
        BetOfferProcessor {
            payload: String::from("BetOfferProcessor"),
        }
    }
}

impl Formatter for BetOfferProcessor{
    fn formater(&self) -> String {
        todo!()
    }
}