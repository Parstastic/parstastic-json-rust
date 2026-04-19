use std::marker::PhantomData;

use crate::{
    node::json_particle::JsonParticle,
    parser::{
        json_parsing_process::JsonParsingProcess,
        json_parsing_result::{
            JsonParsingResult,
            JsonParsingResultError
        },
        parsers::json_particle_parser::JsonParticleParser,
        steps::json_parsing_step::JsonParsingStep
    }
};

pub struct ParseStep<T: JsonParticle, P: JsonParticleParser<T, PS>, PS: JsonParsingStep, S: JsonParsingStep> {
    parser: P,
    _parser_step: PhantomData<PS>,
    next_creator: Box<dyn Fn(T) -> S>
}

impl<T: JsonParticle, P: JsonParticleParser<T, PS>, PS: JsonParsingStep, S: JsonParsingStep> ParseStep<T, P, PS, S> {
    pub fn new<F>(parser: P, next_creator: F) -> Self
        where F: Fn(T) -> S + 'static
    {
        Self {
            parser,
            _parser_step: PhantomData,
            next_creator: Box::new(next_creator)
        }
    }
}

impl<T: JsonParticle, P: JsonParticleParser<T, PS>, PS: JsonParsingStep, S: JsonParsingStep> JsonParsingStep for ParseStep<T, P, PS, S> {
    fn execute(&self, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        if self.parser.can_parse(parsing_process) {
            let result = self.parser.parse(parsing_process);
            match result {
                JsonParsingResult::Value(value) => (self.next_creator)(value).execute(parsing_process),
                JsonParsingResult::Error(_) => Some(JsonParsingResultError::new(
                    "An error occurred during usage of the parser.".to_string(),
                    parsing_process.clone()
                )),
            }
        } else {
            Some(JsonParsingResultError::new(
                "The given parser cannot parse the JsonParsingProcess.".to_string(),
                parsing_process.clone()
            ))
        }
    }
}
