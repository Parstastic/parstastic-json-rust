use crate::node::{
    json_particle::JsonParticle,
    nodes::json_node::JsonNode,
    stringify_options::{
        StringifyOptions,
        StringifyOptionsContainer
    },
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
    stringify_options_type: StringifyOptionsContainer,
}

impl<P: JsonParticle> ContainerNode<P> {
    pub fn new(
        value: ContainerNodeValue<P>,
        delimiter_start: char,
        delimiter_end: char,
        delimiter_elements: char,
        stringify_options_type: StringifyOptionsContainer,
    ) -> Self {
        Self {
            value,
            delimiter_start,
            delimiter_end,
            delimiter_elements,
            stringify_options_type,
        }
    }

    pub fn get_value(self) -> ContainerNodeValue<P> {
        self.value
    }
}

impl<P: JsonParticle> JsonNode for ContainerNode<P> {

}

impl<P: JsonParticle> JsonParticle for ContainerNode<P> {
    fn stringify_with_options(&self, options: &StringifyOptions) -> String {
        let options_for_this_node = options.for_container_node(self.stringify_options_type);
        let mut s = String::new();
        s.push(self.delimiter_start);
        match &self.value {
            ContainerNodeValue::Whitespace(whitespace) => {
                s.push_str(&options_for_this_node.get_container_node_whitespace(whitespace).stringify_with_options(&options_for_this_node));
            },
            ContainerNodeValue::Elements(items) => {
                for (i, val) in items.iter().enumerate() {
                    let is_last_element = i == items.len() - 1;
                    let options_for_this_element = if is_last_element {
                        &options.for_container_node_last_element(self.stringify_options_type)
                    } else {
                        &options_for_this_node
                    };
                    s.push_str(&val.stringify_with_options(options_for_this_element));
                    if !is_last_element {
                        s.push(self.delimiter_elements);
                    }
                }
            },
        }
        s.push(self.delimiter_end);
        s
    }
}
