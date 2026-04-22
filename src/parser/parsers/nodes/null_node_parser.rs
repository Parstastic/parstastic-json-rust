use std::array::from_fn;

use crate::{
    node::nodes::null_node::{
        NULL_NODE,
        NullNode,
        STRING_VALUE
    },
    parser::{
        json_parsing_process::JsonParsingProcess,
        parsers::{
            json_particle_parser::JsonParticleParser,
            nodes::json_node_parser::JsonNodeParser
        },
        steps::{
            block_step::BlockStep,
            json_parsing_step::JsonParsingStep,
            parse_character_step::ParseCharacterStep
        }
    }
};

const N: usize = STRING_VALUE.len();

pub struct NullNodeParser {

}

impl NullNodeParser {
    pub fn new() -> Self {
        Self {

        }
    }

    fn create_validation(&self) -> [Box<dyn JsonParsingStep<NullNode, Self>>; N] {
        let mut iter = STRING_VALUE.chars();
        from_fn(|_| {
            Box::new(
                ParseCharacterStep::new_with_expected_character(iter.next().unwrap())
            ) as Box<dyn JsonParsingStep<NullNode, Self>>
        })
    }
}

impl JsonNodeParser<NullNode> for NullNodeParser {

}

impl JsonParticleParser<NullNode> for NullNodeParser {
    type Step = BlockStep<N, NullNode, Self>;

    fn can_parse(&self, parsing_process: &JsonParsingProcess) -> bool {
        parsing_process.starts_with(STRING_VALUE)
    }

    fn get_step(&mut self) -> Self::Step {
        BlockStep::new(self.create_validation())
    }

    fn create(&self) -> Option<NullNode> {
        Some(NULL_NODE)
    }
}
