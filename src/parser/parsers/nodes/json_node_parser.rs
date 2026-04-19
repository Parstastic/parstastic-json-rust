use crate::{
    node::json_particle::JsonParticle,
    parser::{
        parsers::json_particle_parser::JsonParticleParser,
        steps::json_parsing_step::JsonParsingStep
    }
};

pub trait JsonNodeParser<T: JsonParticle, S: JsonParsingStep>: JsonParticleParser<T, S> {
    
}
