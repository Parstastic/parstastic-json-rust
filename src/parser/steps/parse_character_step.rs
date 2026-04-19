use crate::parser::{
    json_parsing_process::JsonParsingProcess,
    json_parsing_result::JsonParsingResultError,
    steps::json_parsing_step::JsonParsingStep
};

pub struct ParseCharacterStep {
    exporter: Box<dyn Fn(char) -> bool>
}

impl ParseCharacterStep {
    pub fn new<F>(exporter: F) -> Self
        where F: Fn(char) -> bool + 'static
    {
        Self {
            exporter: Box::new(exporter)
        }
    }

    pub fn new_with_expected_character(character: char) -> Self {
        Self::new(move |c| c == character)
    }
}

impl JsonParsingStep for ParseCharacterStep {
    fn execute(&self, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        if parsing_process.is_index_in_json() {
            if parsing_process.is_char_valid(&self.exporter) {
                parsing_process.increment_index();
                None
            } else {
                Some(JsonParsingResultError::new(
                    "The required character to parse was not found.".to_string(),
                    parsing_process.clone()
                ))
            }
        } else {
            Some(JsonParsingResultError::new(
                "The JsonParsingProcess has exceeded its JSON's length.".to_string(),
                parsing_process.clone()
            ))
        }
    }
}
