use crate::parser::{
    json_parsing_process::JsonParsingProcess,
    json_parsing_result::JsonParsingResultError,
    steps::json_parsing_step::JsonParsingStep
};

pub struct ExportStep<F: Fn() -> bool> {
    exporter: F
}

impl<F: Fn() -> bool> ExportStep<F> {
    pub fn new(exporter: F) -> Self {
        Self {
            exporter: exporter
        }
    }
}

impl<F: Fn() -> bool> JsonParsingStep for ExportStep<F> {
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
