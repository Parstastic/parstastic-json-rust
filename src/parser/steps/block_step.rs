use crate::{
    node::json_particle::JsonParticle, 
    parser::{
        json_parsing_process::JsonParsingProcess,
        json_parsing_result::JsonParsingResultError,
        parsers::json_particle_parser::JsonParticleParser,
        steps::json_parsing_step::JsonParsingStep
    }
};

pub struct BlockStep<const N: usize, JP: JsonParticle, JPP: JsonParticleParser<JP>> {
    instructions: [Box<dyn JsonParsingStep<JP, JPP>>; N]
}

impl<const N: usize, JP: JsonParticle, JPP: JsonParticleParser<JP>> BlockStep<N, JP, JPP> {
    pub fn new(instructions: [Box<dyn JsonParsingStep<JP, JPP>>; N]) -> Self {
        Self {
            instructions
        }
    }
}

impl<const N: usize, JP: JsonParticle, JPP: JsonParticleParser<JP>> JsonParsingStep<JP, JPP> for BlockStep<N, JP, JPP> {
    fn execute(&self, parser: &mut JPP, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        for instruction in self.instructions.iter() {
            let result = instruction.execute(parser, parsing_process);
            if result.is_some() {
                return result;
            }
        }
        None
    }
}
