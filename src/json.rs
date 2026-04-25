use crate::{
    node::{
        json_particle::JsonParticle,
        json_value::JsonValue
    },
    parser::{
        full_string_json_parser::FullStringJsonParser,
        json_parsing_result::JsonParsingResult
    }
};

pub struct JSON {
    _private: ()
}

impl JSON {
    pub fn stringify<J: JsonParticle>(json_particle: J) -> String {
        json_particle.stringify()
    }

    pub fn parse(json: String) -> Option<JsonValue> {
        match FullStringJsonParser::new().parse_string_fully(json) {
            JsonParsingResult::Value(v) => Some(v),
            JsonParsingResult::Error(_) => None,
        }
    }

    pub fn parse_unsafe(json: String) -> JsonValue {
        Self::parse(json).expect("An error occurred during parsing of JSON.")
    }
}
