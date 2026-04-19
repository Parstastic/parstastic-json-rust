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

pub struct WhileLoopStep<T: JsonParsingStep<JP, JPP>, JP: JsonParticle, JPP: JsonParticleParser<JP>> {
    _jp: PhantomData<JP>,
    _jpp: PhantomData<JPP>,
    instruction: T,
    continue_criteria: Box<dyn Fn(&JsonParsingProcess) -> bool>
}

impl<T: JsonParsingStep<JP, JPP>, JP: JsonParticle, JPP: JsonParticleParser<JP>> WhileLoopStep<T, JP, JPP> {
    pub fn new<F>(instruction: T, continue_criteria: F) -> Self
        where F: Fn(&JsonParsingProcess) -> bool + 'static
    {
        Self {
            _jp: PhantomData,
            _jpp: PhantomData,
            instruction,
            continue_criteria: Box::new(continue_criteria)
        }
    }
}

impl<T: JsonParsingStep<JP, JPP>, JP: JsonParticle, JPP: JsonParticleParser<JP>> JsonParsingStep<JP, JPP> for WhileLoopStep<T, JP, JPP> {
    fn execute(&mut self, parser: &mut JPP, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        while (self.continue_criteria)(parsing_process) {
            let result = self.instruction.execute(parser, parsing_process);
            if result.is_some() {
                return result;
            }
        }
        None
    }
}

impl<T: JsonParsingStep<JP, JPP>, JP: JsonParticle, JPP: JsonParticleParser<JP>> LoopStep<JP, JPP> for WhileLoopStep<T, JP, JPP> {
    
}
