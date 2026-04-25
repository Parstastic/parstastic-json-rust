use crate::{
    node::json_value::JsonValue,
    parser::{
        json_parsing_process::JsonParsingProcess,
        json_parsing_result::JsonParsingResult,
        parsers::{
            json_particle_parser::JsonParticleParser,
            json_value_parser::JsonValueParser
        },
        steps::parse_step::ParseStep
    }
};

pub struct FullStringJsonParser {
    json_value: Option<JsonValue>
}

impl FullStringJsonParser {
    pub fn new() -> Self {
        Self {
            json_value: None
        }
    }

    pub fn parse_fully(self, parsing_process: &mut JsonParsingProcess) -> JsonParsingResult<JsonValue> {
        let result = self.parse(parsing_process);
        match result {
            JsonParsingResult::Value(value) if parsing_process.is_finished() => JsonParsingResult::Value(value),
            JsonParsingResult::Error(error) => JsonParsingResult::Error(error),
            _ => JsonParsingResult::with_error_to_create(
                "The JSON String is not fully parsed.".to_string(),
                parsing_process.clone()
            ),
        }
    }
}

impl JsonParticleParser<JsonValue> for FullStringJsonParser {
    type Step = ParseStep<JsonValue, JsonValueParser, JsonValue, Self>;

    fn can_parse(&self, parsing_process: &JsonParsingProcess) -> bool {
        JsonValueParser::new().can_parse(parsing_process)
    }

    fn get_step(&mut self) -> Self::Step {
        ParseStep::new(
            |_: &mut Self| JsonValueParser::new(),
            |v, p, _| {
                p.json_value = Some(v);
                None
            }
        )
    }

    fn create(self) -> Option<JsonValue> {
        self.json_value
    }
}
