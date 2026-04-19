use crate::{
    node::json_particle::JsonParticle,
    parser::{
        json_parsing_process::JsonParsingProcess,
        json_parsing_result::JsonParsingResultError,
        parsers::json_particle_parser::JsonParticleParser
    }
};

pub trait JsonParsingStep<JP: JsonParticle, JPP: JsonParticleParser<JP>> {
    fn execute(&self, parser: &mut JPP, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError>;
}
