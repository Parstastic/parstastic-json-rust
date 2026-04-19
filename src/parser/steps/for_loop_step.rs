use std::marker::PhantomData;

use crate::{
    node::json_particle::JsonParticle, 
    parser::{
        json_parsing_process::JsonParsingProcess, 
        json_parsing_result::JsonParsingResultError, 
        parsers::json_particle_parser::JsonParticleParser, 
        steps::{
            json_parsing_step::JsonParsingStep, 
            loop_step::LoopStep
        }
    }
};

pub struct ForLoopStep<T: JsonParsingStep<JP, JPP>, JP: JsonParticle, JPP: JsonParticleParser<JP>> {
    _jp: PhantomData<JP>,
    _jpp: PhantomData<JPP>,
    instruction: T,
    iterations: u32
}

impl<T: JsonParsingStep<JP, JPP>, JP: JsonParticle, JPP: JsonParticleParser<JP>> ForLoopStep<T, JP, JPP> {
    pub fn new(instruction: T, iterations: u32) -> Self {
        Self {
            _jp: PhantomData,
            _jpp: PhantomData,
            instruction,
            iterations
        }
    }
}

impl<T: JsonParsingStep<JP, JPP>, JP: JsonParticle, JPP: JsonParticleParser<JP>> JsonParsingStep<JP, JPP> for ForLoopStep<T, JP, JPP> {
    fn execute(&mut self, parser: &mut JPP, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        for _ in 0..self.iterations {
            let result = self.instruction.execute(parser, parsing_process);
            if result.is_some() {
                return result;
            }
        }
        None
    }
}

impl<T: JsonParsingStep<JP, JPP>, JP: JsonParticle, JPP: JsonParticleParser<JP>> LoopStep<JP, JPP> for ForLoopStep<T, JP, JPP> {
    
}
