use crate::parser::{
    json_parsing_process::JsonParsingProcess,
    json_parsing_result::JsonParsingResultError,
    steps::json_parsing_step::JsonParsingStep
};

pub struct ExportStep {
    exporter: Box<dyn Fn() -> bool>
}

impl ExportStep {
    pub fn new<F>(exporter: F) -> Self
        where F: Fn() -> bool + 'static
    {
        Self {
            exporter: Box::new(exporter)
        }
    }
}

impl JsonParsingStep for ExportStep {
    fn execute(&self, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        if (self.exporter)() {
            None
        } else {
            Some(JsonParsingResultError::new(
                "Exporting failed".to_string(),
                parsing_process.clone()
            ))
        }
    }
}
