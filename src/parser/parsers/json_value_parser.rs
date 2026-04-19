use crate::{
    node::{
        json_value::JsonValue, 
        nodes::json_node::JsonNode, 
        whitespace::Whitespace
    },
    parser::{
        parsers::json_particle_parser::JsonParticleParser, 
        steps::block_step::BlockStep
    }
};

pub struct JsonValueParser {
    leading_whitespace: Option<Whitespace>,
    json_node: Option<Box<dyn JsonNode>>,
    trailing_whitespace: Option<Whitespace>,
}

impl JsonValueParser {
    pub fn new() -> Self {
        Self {
            leading_whitespace: None,
            json_node: None,
            trailing_whitespace: None,
        }
    }

    pub fn new_with_leading_whitespace(leading_whitespace: Whitespace) -> Self {
        Self {
            leading_whitespace: Some(leading_whitespace),
            json_node: None,
            trailing_whitespace: None,
        }
    }
}

impl JsonParticleParser<JsonValue> for JsonValueParser {
    type Step = BlockStep<3, JsonValue, Self>;

    fn can_parse(&self, parsing_process: &crate::parser::json_parsing_process::JsonParsingProcess) -> bool {
        todo!()
    }

    fn get_step(&mut self) -> Self::Step {
        todo!()
    }

    fn create(&self) -> Option<JsonValue> {
        todo!()
    }
}
