use crate::{
    node::{
        json_particle::JsonParticle, 
        nodes::boolean_node::{
            BOOLEAN_NODES, 
            BooleanNode, 
            FALSE, 
            TRUE
        }
    },
    parser::{
        json_parsing_process::JsonParsingProcess,
        parsers::{
            json_particle_parser::JsonParticleParser,
            nodes::json_node_parser::JsonNodeParser
        },
        steps::{
            block_step::BlockStep,
            export_step::ExportStep,
            for_loop_step::ForLoopStep,
            json_parsing_step::JsonParsingStep,
            or_step::OrStep,
            parse_character_step::ParseCharacterStep
        }
    }
};

pub struct BooleanNodeParser {
    value: Option<bool>
}

impl BooleanNodeParser {
    pub fn new() -> Self {
        Self {
            value: None
        }
    }

    fn create_parsers_map(&self) -> Vec<(Box<dyn Fn(&JsonParsingProcess) -> bool>, Box<dyn JsonParsingStep<BooleanNode, Self>>)> {
        BOOLEAN_NODES.iter()
            .map(|n| self.create_entry(n))
            .collect()
    }

    fn create_entry(&self, n: &BooleanNode) -> (Box<dyn Fn(&JsonParsingProcess) -> bool>, Box<dyn JsonParsingStep<BooleanNode, Self>>) {
        let string_value = n.stringify();
        let len = string_value.len();
        let value = n.get_value();
        (
            Box::new(move |p| p.starts_with(&string_value)),
            Box::new(BlockStep::new([
                Box::new(ForLoopStep::new(
                    ParseCharacterStep::new(|_, _| true),
                    len as u32
                )),
                Box::new(ExportStep::new_with_value(|p: &mut BooleanNodeParser, v| {
                    p.value = Some(*v);
                    true
                }, value))
            ]))
        )
    }
}

impl JsonNodeParser<BooleanNode> for BooleanNodeParser {

}

impl JsonParticleParser<BooleanNode> for BooleanNodeParser {
    type Step = OrStep<BooleanNode, Self>;

    fn can_parse(&self, parsing_process: &JsonParsingProcess) -> bool {
        BOOLEAN_NODES.iter()
            .any(|n| parsing_process.starts_with(&n.stringify()))
    }

    fn get_step(&mut self) -> Self::Step {
        OrStep::else_error(
            self.create_parsers_map()
        )
    }

    fn create(&self) -> Option<BooleanNode> {
        let value = self.value?;
        Some(if value {
            TRUE
        } else {
            FALSE
        })
    }
}
