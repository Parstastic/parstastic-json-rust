use crate::{
    node::json_particle::JsonParticle,
    parser::{
        json_parsing_process::JsonParsingProcess,
        json_parsing_result::JsonParsingResult,
        steps::json_parsing_step::JsonParsingStep
    }
};

pub trait JsonParticleParser<T: JsonParticle, S: JsonParsingStep> {
    fn can_parse_string(&self, json: String) -> bool {
        self.can_parse(&JsonParsingProcess::new_for_json(json))
    }

    fn can_parse(&self, parsing_process: &JsonParsingProcess) -> bool;

    fn parse_string(&self, json: String) -> JsonParsingResult<T> {
        self.parse(&mut JsonParsingProcess::new_for_json(json))
    }

    fn parse(&self, parsing_process: &mut JsonParsingProcess) -> JsonParsingResult<T> {
        let step = self.get_step();
        let result = step.execute(parsing_process);
        match result {
            Some(error) => JsonParsingResult::with_error(error),
            None => JsonParsingResult::with_value(self.create()),
        }
    }

    fn get_step(&self) -> S;

    fn create(&self) -> T;
}
