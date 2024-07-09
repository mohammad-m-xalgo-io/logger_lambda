use crate::services::formatters::{S3PayloadFormatter, Formatter};
pub struct OutcomeProcessor {
    pub payload: String,
}

impl OutcomeProcessor {
    pub fn new() -> OutcomeProcessor {
        OutcomeProcessor {
            payload: String::from("OutcomeProcessor"),
        }
    }
}
impl Formatter for OutcomeProcessor {
    fn formater(&self) -> String {
        todo!()
    }
}