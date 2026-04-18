use crate::node::{
    json_particle::JsonParticle,
    nodes::json_node::JsonNode,
    stringify_options::StringifyOptions,
    whitespace::Whitespace
};

pub enum ContainerNodeValue<P: JsonParticle> {
    Whitespace(Whitespace),
    Elements(Vec<P>),
}


pub struct ContainerNode<P: JsonParticle> {
    value: ContainerNodeValue<P>,
    delimiter_start: char,
    delimiter_end: char,
    delimiter_elements: char,
}

impl<P: JsonParticle> ContainerNode<P> {
    pub(super) fn new(
        value: ContainerNodeValue<P>,
        delimiter_start: char,
        delimiter_end: char,
        delimiter_elements: char,
    ) -> Self {
        Self {
            value,
            delimiter_start,
            delimiter_end,
            delimiter_elements,
        }
    }
}

impl<P: JsonParticle> JsonNode for ContainerNode<P> {

}

impl<P: JsonParticle> JsonParticle for ContainerNode<P> {
    fn stringify_with_options(&self, options: &StringifyOptions) -> String {
        todo!()
    }
}
