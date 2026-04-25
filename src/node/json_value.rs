use crate::node::{
    json_particle::JsonParticle,
    nodes::{
        array_node::ArrayNode,
        boolean_node::BooleanNode,
        null_node::NullNode,
        number_node::NumberNode,
        object_node::ObjectNode,
        string_node::StringNode
    },
    stringify_options::StringifyOptions,
    whitespace::Whitespace
};

pub enum JsonValueJsonNodeType {
    StringNode(StringNode),
    NumberNode(NumberNode),
    ObjectNode(ObjectNode),
    ArrayNode(ArrayNode),
    BooleanNode(BooleanNode),
    NullNode(NullNode),
}

impl JsonValueJsonNodeType {
    pub fn stringify(&self) -> String {
        self.stringify_with_options(&StringifyOptions::default())
    }

    pub fn stringify_with_options(&self, options: &StringifyOptions) -> String {
        match self {
            JsonValueJsonNodeType::StringNode(string_node) => string_node.stringify_with_options(options),
            JsonValueJsonNodeType::NumberNode(number_node) => number_node.stringify_with_options(options),
            JsonValueJsonNodeType::ObjectNode(object_node) => object_node.stringify_with_options(options),
            JsonValueJsonNodeType::ArrayNode(array_node) => array_node.stringify_with_options(options),
            JsonValueJsonNodeType::BooleanNode(boolean_node) => boolean_node.stringify_with_options(options),
            JsonValueJsonNodeType::NullNode(null_node) => null_node.stringify_with_options(options),
        }
    }
}


pub struct JsonValue {
    leading_whitespace: Whitespace,
    json_node: JsonValueJsonNodeType,
    trailing_whitespace: Whitespace
}

impl JsonValue {
    pub fn new(leading_whitespace: Whitespace, json_node: JsonValueJsonNodeType, trailing_whitespace: Whitespace) -> Self {
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
