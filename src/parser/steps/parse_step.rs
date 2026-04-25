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

pub struct ParseStep<
    T: JsonParticle, P: JsonParticleParser<T>,
    JP: JsonParticle, JPP: JsonParticleParser<JP>
> {
    _jp: PhantomData<JP>,
    _jpp: PhantomData<JPP>,
    parser_creator: Box<dyn Fn(&mut JPP) -> P>,
    next_creator: Box<dyn Fn(T, &mut JPP, &mut JsonParsingProcess) -> Option<JsonParsingResultError>>
}

impl<
    T: JsonParticle, P: JsonParticleParser<T>,
    JP: JsonParticle, JPP: JsonParticleParser<JP>
> ParseStep<T, P, JP, JPP> {
    pub fn new<F1, F2>(parser_creator: F1, next_creator: F2) -> Self
        where
            F1: Fn(&mut JPP) -> P + 'static,
            F2: Fn(T, &mut JPP, &mut JsonParsingProcess) -> Option<JsonParsingResultError> + 'static
    {
        Self {
            _jp: PhantomData,
            _jpp: PhantomData,
            parser_creator: Box::new(parser_creator),
            next_creator: Box::new(next_creator)
        }
    }
}

impl<
    T: JsonParticle, P: JsonParticleParser<T>,
    JP: JsonParticle, JPP: JsonParticleParser<JP>
> JsonParsingStep<JP, JPP> for ParseStep<T, P, JP, JPP> {
    fn execute(&self, parser: &mut JPP, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        let new_parser = (self.parser_creator)(parser);
        if new_parser.can_parse(parsing_process) {
            let result = new_parser.parse(parsing_process);
            match result {
                JsonParsingResult::Value(value) => (self.next_creator)(value, parser, parsing_process),
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
