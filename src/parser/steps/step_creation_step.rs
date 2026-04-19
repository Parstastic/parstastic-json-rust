use std::marker::PhantomData;

use crate::{
    node::json_particle::JsonParticle, 
    parser::{
        json_parsing_process::JsonParsingProcess,
        json_parsing_result::JsonParsingResultError,
        parsers::json_particle_parser::JsonParticleParser,
        steps::json_parsing_step::JsonParsingStep
    }
};

pub struct StepCreationStep<T: JsonParsingStep<JP, JPP>, JP: JsonParticle, JPP: JsonParticleParser<JP>> {
    _jp: PhantomData<JP>,
    _jpp: PhantomData<JPP>,
    step_creator: Box<dyn Fn() -> T>
}

impl<T: JsonParsingStep<JP, JPP>, JP: JsonParticle, JPP: JsonParticleParser<JP>> StepCreationStep<T, JP, JPP> {
    pub fn new<F>(step_creator: F) -> Self
        where F: Fn() -> T + 'static
    {
        Self {
            _jp: PhantomData,
            _jpp: PhantomData,
            step_creator: Box::new(step_creator)
        }
    }
}

impl<T: JsonParsingStep<JP, JPP>, JP: JsonParticle, JPP: JsonParticleParser<JP>> JsonParsingStep<JP, JPP> for StepCreationStep<T, JP, JPP> {
    fn execute(&self, parser: &mut JPP, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        (self.step_creator)().execute(parser, parsing_process)
    }
}
