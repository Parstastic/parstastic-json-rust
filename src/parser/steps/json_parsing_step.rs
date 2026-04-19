use crate::parser::{
    json_parsing_process::JsonParsingProcess,
    json_parsing_result::JsonParsingResultError
};

pub trait JsonParsingStep {
    fn execute(&self, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError>;
}
