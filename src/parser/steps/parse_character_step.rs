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

pub struct ParseCharacterStep<JP: JsonParticle, JPP: JsonParticleParser<JP>> {
    _jp: PhantomData<JP>,
    exporter: Box<dyn Fn(&mut JPP, char) -> bool>
}

impl<JP: JsonParticle, JPP: JsonParticleParser<JP>> ParseCharacterStep<JP, JPP> {
    pub fn new<F>(exporter: F) -> Self
        where F: Fn(&mut JPP, char) -> bool + 'static
    {
        Self {
            _jp: PhantomData,
            exporter: Box::new(exporter)
        }
    }

    pub fn new_with_expected_character(character: char) -> Self {
        Self::new(move |_jpp, c| c == character)
    }
}

impl<JP: JsonParticle, JPP: JsonParticleParser<JP>> JsonParsingStep<JP, JPP> for ParseCharacterStep<JP, JPP> {
    fn execute(&self, parser: &mut JPP, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        if parsing_process.is_index_in_json() {
            match parsing_process.get_char() {
                Some(char) => {
                    (self.exporter)(parser, char);
                    parsing_process.increment_index();
                    None
                },
                None => Some(JsonParsingResultError::new(
                    "The required character to parse was not found.".to_string(),
                    parsing_process.clone()
                )),
            }
        } else {
            Some(JsonParsingResultError::new(
                "The JsonParsingProcess has exceeded its JSON's length.".to_string(),
                parsing_process.clone()
            ))
        }
    }
}
