use crate::node::{
    json_particle::JsonParticle,
    json_value::JsonValue,
    nodes::{
        container_node::{
            ContainerNode,
            ContainerNodeValue
        },
        json_node::JsonNode,
        string_node::StringNode
    },
    stringify_options::StringifyOptions,
    whitespace::Whitespace
};

pub const KEY_VALUE_DELIMITER: char = ':';

pub struct ObjectNodeProperty {
    leading_whitespace: Whitespace,
    key: StringNode,
    trailing_whitespace: Whitespace,
    value: JsonValue,
}

impl ObjectNodeProperty {
    pub fn new(
        leading_whitespace: Whitespace,
        key: StringNode,
        trailing_whitespace: Whitespace,
        value: JsonValue,
    ) -> Self {
        Self {
            leading_whitespace,
            key,
            trailing_whitespace,
            value,
        }
    }
}

impl JsonParticle for ObjectNodeProperty {
    fn stringify_with_options(&self, options: &StringifyOptions) -> String {
        todo!()
    }
}


pub const DELIMITER_START: char = '{';
pub const DELIMITER_END: char = '}';
pub const DELIMITER_ELEMENTS: char = ',';


pub struct ObjectNode {
    container_node: ContainerNode<ObjectNodeProperty>
}

impl ObjectNode {
    pub fn new_with_whitespace(whitespace: Whitespace) -> Self {
        Self {
            container_node: ContainerNode::new(
                ContainerNodeValue::Whitespace(whitespace),
                DELIMITER_START,
                DELIMITER_END,
                DELIMITER_ELEMENTS
            )
        }
    }

    pub fn new_with_elements(elements: Vec<ObjectNodeProperty>) -> Self {
        Self {
            container_node: ContainerNode::new(
                ContainerNodeValue::Elements(elements),
                DELIMITER_START,
                DELIMITER_END,
                DELIMITER_ELEMENTS
            )
        }
    }
}

impl JsonNode for ObjectNode {

}

impl JsonParticle for ObjectNode {
    fn stringify_with_options(&self, options: &StringifyOptions) -> String {
        self.container_node.stringify_with_options(options)
    }
}
