use crate::{
    node::json_particle::JsonParticle,
    parser::{
        json_parsing_process::JsonParsingProcess,
        json_parsing_result::JsonParsingResultError,
        parsers::json_particle_parser::JsonParticleParser,
        steps::json_parsing_step::JsonParsingStep
    }
};

pub struct ExportStep {
    exporter: Box<dyn Fn() -> bool>
}

impl ExportStep {
    pub fn new<F>(exporter: F) -> Self
        where F: Fn() -> bool + 'static
    {
        Self {
            exporter: Box::new(exporter)
        }
    }
}

impl<JP: JsonParticle, JPP: JsonParticleParser<JP,>> JsonParsingStep<JP, JPP> for ExportStep {
    fn execute(&mut self, _parser: &mut JPP, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        if (self.exporter)() {
            None
        } else {
            Some(JsonParsingResultError::new(
                "Exporting failed".to_string(),
                parsing_process.clone()
            ))
        }
    }
}
