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
    stringify_options::{
        StringifyOptions,
        StringifyOptionsContainer
    },
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
    type Value = (Whitespace, StringNode, Whitespace, JsonValue);
    
    type BorrowedValue<'a> = (&'a Whitespace, &'a StringNode, &'a Whitespace, &'a JsonValue)
        where Self: 'a;
    
    fn extract_value(self) -> Self::Value {
        (
            self.leading_whitespace,
            self.key,
            self.trailing_whitespace,
            self.value
        )
    }
    
    fn get_value<'a>(&'a self) -> Self::BorrowedValue<'a> {
        (
            &self.leading_whitespace,
            &self.key,
            &self.trailing_whitespace,
            &self.value
        )
    }
    
    fn stringify_with_options(&self, options: &StringifyOptions) -> String {
        let mut s = String::new();
        s.push_str(&options.get_object_node_property_leading_whitespace(&self.leading_whitespace).stringify_with_options(options));
        s.push_str(&self.key.stringify_with_options(options));
        s.push_str(&options.get_object_node_property_trailing_whitespace(&self.trailing_whitespace).stringify_with_options(options));
        s.push(KEY_VALUE_DELIMITER);
        s.push_str(&self.value.stringify_with_options(options));
        s
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
                DELIMITER_ELEMENTS,
                StringifyOptionsContainer::ObjectNode,
            )
        }
    }

    pub fn new_with_elements(elements: Vec<ObjectNodeProperty>) -> Self {
        Self {
            container_node: ContainerNode::new(
                ContainerNodeValue::Elements(elements),
                DELIMITER_START,
                DELIMITER_END,
                DELIMITER_ELEMENTS,
                StringifyOptionsContainer::ObjectNode,
            )
        }
    }
}

impl JsonNode for ObjectNode {

}

impl JsonParticle for ObjectNode {
    type Value = ContainerNodeValue<ObjectNodeProperty>;
    
    type BorrowedValue<'a> = &'a ContainerNodeValue<ObjectNodeProperty>
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
