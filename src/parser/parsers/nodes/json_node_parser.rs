use crate::{
    node::json_particle::JsonParticle,
    parser::parsers::json_particle_parser::JsonParticleParser
};

pub trait JsonNodeParser<T: JsonParticle>: JsonParticleParser<T> {
    
}
