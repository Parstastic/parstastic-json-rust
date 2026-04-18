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
    fn stringify_with_options(&self, options: &StringifyOptions) -> String {
        let mut s = String::new();
        s.push_str(&options.get_json_value_leading_whitespace(&self.leading_whitespace).stringify_with_options(&options));
        s.push_str(&self.json_node.stringify_with_options(&options));
        s.push_str(&options.get_json_value_trailing_whitespace(&self.trailing_whitespace).stringify_with_options(&options));
        s
    }
}
