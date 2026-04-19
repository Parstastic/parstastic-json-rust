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

pub struct ExportStep<JP: JsonParticle, JPP: JsonParticleParser<JP>> {
    _jp: PhantomData<JP>,
    exporter: Box<dyn Fn(&mut JPP) -> bool>
}

impl<JP: JsonParticle, JPP: JsonParticleParser<JP>> ExportStep<JP, JPP> {
    pub fn new<F>(exporter: F) -> Self
        where F: Fn(&mut JPP) -> bool + 'static
    {
        Self {
            _jp: PhantomData,
            exporter: Box::new(exporter)
        }
    }
}

impl<JP: JsonParticle, JPP: JsonParticleParser<JP,>> JsonParsingStep<JP, JPP> for ExportStep<JP, JPP> {
    fn execute(&self, parser: &mut JPP, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        if (self.exporter)(parser) {
            None
        } else {
            Some(JsonParsingResultError::new(
                "Exporting failed".to_string(),
                parsing_process.clone()
            ))
        }
    }
}
