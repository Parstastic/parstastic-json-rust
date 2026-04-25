use crate::node::{
    json_particle::JsonParticle,
    json_value::JsonValue,
    nodes::{
        container_node::{
            ContainerNode,
            ContainerNodeValue
        },
        json_node::JsonNode
    },
    stringify_options::{
        StringifyOptions,
        StringifyOptionsContainer
    },
    whitespace::Whitespace
};

pub const DELIMITER_START: char = '[';
pub const DELIMITER_END: char = ']';
pub const DELIMITER_ELEMENTS: char = ',';


pub struct ArrayNode {
    container_node: ContainerNode<JsonValue>
}

impl ArrayNode {
    pub fn new_with_whitespace(whitespace: Whitespace) -> Self {
        Self {
            container_node: ContainerNode::new(
                ContainerNodeValue::Whitespace(whitespace),
                DELIMITER_START,
                DELIMITER_END,
                DELIMITER_ELEMENTS,
                StringifyOptionsContainer::ArrayNode,
            )
        }
    }

    pub fn new_with_elements(elements: Vec<JsonValue>) -> Self {
        Self {
            container_node: ContainerNode::new(
                ContainerNodeValue::Elements(elements),
                DELIMITER_START,
                DELIMITER_END,
                DELIMITER_ELEMENTS,
                StringifyOptionsContainer::ArrayNode,
            )
        }
    }
}

impl JsonNode for ArrayNode {

}

impl JsonParticle for ArrayNode {
    type Value = ContainerNodeValue<JsonValue>;
    
    type BorrowedValue<'a> = &'a ContainerNodeValue<JsonValue>
        where Self: 'a;
    
    fn extract_value(self) -> Self::Value {
        self.container_node.extract_value()
    }
    
    fn get_value<'a>(&'a self) -> Self::BorrowedValue<'a> {
        &self.container_node.get_value()
    }

    fn stringify_with_options(&self, options: &StringifyOptions) -> String {
        self.container_node.stringify_with_options(options)
    }
}
