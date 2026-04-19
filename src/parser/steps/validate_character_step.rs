use crate::parser::{
    json_parsing_process::JsonParsingProcess,
    json_parsing_result::JsonParsingResultError,
    steps::json_parsing_step::JsonParsingStep
};

pub struct ValidateCharacterStep {
    validator: Box<dyn Fn(char) -> bool>
}

impl ValidateCharacterStep {
    pub fn new<F>(validator: F) -> Self
        where F: Fn(char) -> bool + 'static
    {
        Self {
            validator: Box::new(validator)
        }
    }

    pub fn new_with_expected_character(character: char) -> Self {
        Self::new(move |c| c == character)
    }
}

impl JsonParsingStep for ValidateCharacterStep {
    fn execute(&self, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        if parsing_process.is_char_valid(&self.validator) {
            None
        } else {
            Some(JsonParsingResultError::new(
                "The character to validate was not found or invalid.".to_string(),
                parsing_process.clone()
            ))
        }
    }
}
