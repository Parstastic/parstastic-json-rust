use crate::node::{
    json_particle::JsonParticle,
    nodes::json_node::JsonNode,
    stringify_options::StringifyOptions,
    whitespace::Whitespace
};

pub struct JsonValue<T: JsonNode> {
    leading_whitespace: Whitespace,
    json_node: T,
    trailing_whitespace: Whitespace
}

impl<T: JsonNode> JsonValue<T> {
    pub fn new(leading_whitespace: Whitespace, json_node: T, trailing_whitespace: Whitespace) -> Self {
        Self {
            leading_whitespace,
            json_node,
            trailing_whitespace
        }
    }
}

impl<T: JsonNode> JsonParticle for JsonValue<T> {
    fn stringify_with_options(&self, options: StringifyOptions) -> String {
        todo!()
    }
}
