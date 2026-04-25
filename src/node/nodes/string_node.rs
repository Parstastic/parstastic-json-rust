use crate::node::{
    json_particle::JsonParticle,
    nodes::json_node::JsonNode,
    stringify_options::StringifyOptions
};

pub const DELIMITER: char = '"';


pub struct StringNode {
    value: String
}

impl StringNode {
    pub fn new(value: String) -> Self {
        Self {
            value
        }
    }
}

impl JsonNode for StringNode {

}

impl JsonParticle for StringNode {
    type Value = String;
    
    type BorrowedValue<'a> = &'a str
        where Self: 'a;
    
    fn extract_value(self) -> Self::Value {
        self.value
    }
    
    fn get_value<'a>(&'a self) -> Self::BorrowedValue<'a> {
        &self.value
    }
    
    fn stringify_with_options(&self, _options: &StringifyOptions) -> String {
        let mut s = String::new();
        s.push(DELIMITER);
        s.push_str(&self.value);
        s.push(DELIMITER);
        s
    }
}
