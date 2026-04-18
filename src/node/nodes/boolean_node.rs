use crate::node::{
    json_particle::JsonParticle,
    nodes::json_node::JsonNode,
    stringify_options::StringifyOptions
};

pub struct BooleanNode {
    value: bool
}

impl BooleanNode {
    pub fn has_value(&self, value: bool) -> bool {
        self.value == value
    }
}

impl JsonNode for BooleanNode {

}

impl JsonParticle for BooleanNode {
    fn stringify_with_options(&self, _options: &StringifyOptions) -> String {
        self.value.to_string()
    }
}

pub const TRUE: BooleanNode = BooleanNode {
    value: true
};

pub const FALSE: BooleanNode = BooleanNode {
    value: false
};

pub const BOOLEAN_NODES: [BooleanNode; 2] = [
    TRUE,
    FALSE
];
