use crate::{
    node::json_particle::JsonParticle, 
    parser::{
        parsers::json_particle_parser::JsonParticleParser, 
        steps::json_parsing_step::JsonParsingStep
    }
};

pub trait LoopStep<JP: JsonParticle, JPP: JsonParticleParser<JP>>: JsonParsingStep<JP, JPP> {
    
}
