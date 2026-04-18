use crate::node::{
    json_particle::JsonParticle,
    nodes::json_node::JsonNode,
    stringify_options::StringifyOptions,
    whitespace::Whitespace
};

pub struct JsonValue {
    leading_whitespace: Whitespace,
    json_node: Box<dyn JsonNode>,
    trailing_whitespace: Whitespace
}

impl JsonValue {
    pub fn new(leading_whitespace: Whitespace, json_node: Box<dyn JsonNode>, trailing_whitespace: Whitespace) -> Self {
        Self {
            leading_whitespace,
            json_node,
            trailing_whitespace
        }
    }
}

impl JsonParticle for JsonValue {
    fn stringify_with_options(&self, options: StringifyOptions) -> String {
        todo!()
    }
}
