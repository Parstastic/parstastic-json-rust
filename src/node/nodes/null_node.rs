use crate::node::{
    json_particle::JsonParticle,
    nodes::json_node::JsonNode,
    stringify_options::StringifyOptions
};

pub const STRING_VALUE: &str = "null";


pub struct NullNode {
    _private: ()
}

impl JsonNode for NullNode {

}

impl JsonParticle for NullNode {
    type Value = ();
    
    type BorrowedValue<'a> = &'a ()
        where Self: 'a;
    
    fn extract_value(self) -> Self::Value {
        self._private
    }
    
    fn get_value<'a>(&'a self) -> Self::BorrowedValue<'a> {
        &self._private
    }
    
    fn stringify_with_options(&self, _options: &StringifyOptions) -> String {
        STRING_VALUE.to_string()
    }
}

pub const NULL_NODE: NullNode = NullNode {
    _private: ()
};
