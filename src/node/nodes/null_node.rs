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
    fn stringify_with_options(&self, _options: &StringifyOptions) -> String {
        STRING_VALUE.to_string()
    }
}

pub const NULL_NODE: NullNode = NullNode {
    _private: ()
};
