use crate::{
    node::json_particle::JsonParticle,
    parser::json_parsing_process::JsonParsingProcess
};

pub struct JsonParsingResultError {
    message: String,
    parsing_process: JsonParsingProcess
}

impl JsonParsingResultError {
    pub fn new(message: String, parsing_process: JsonParsingProcess) -> Self {
        Self {
            message,
            parsing_process
        }
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn get_parsing_process(&self) -> &JsonParsingProcess {
        &self.parsing_process
    }
}


pub enum JsonParsingResult<T: JsonParticle> {
    Value(T),
    Error(JsonParsingResultError)
}

impl<T: JsonParticle> JsonParsingResult<T> {
    pub fn with_value(value: T) -> Self {
        Self::Value(value)
    }

    pub fn with_error(error: JsonParsingResultError) -> Self {
        Self::Error(error)
    }

    pub fn with_error_to_create(message: String, parsing_process: JsonParsingProcess) -> Self {
        Self::with_error(JsonParsingResultError::new(message, parsing_process))
    }

    pub fn has_value(&self) -> bool {
        match self {
            JsonParsingResult::Value(_) => true,
            JsonParsingResult::Error(_) => false,
        }
    }

    pub fn get_value(&self) -> Option<&T> {
        match self {
            JsonParsingResult::Value(value) => Some(value),
            JsonParsingResult::Error(_) => None,
        }
    }

    pub fn has_error(&self) -> bool {
        match self {
            JsonParsingResult::Value(_) => false,
            JsonParsingResult::Error(_) => true,
        }
    }

    pub fn get_error(&self) -> Option<&JsonParsingResultError> {
        match self {
            JsonParsingResult::Value(_) => None,
            JsonParsingResult::Error(error) => Some(error),
        }
    }
}
