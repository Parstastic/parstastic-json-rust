use crate::{
    node::json_particle::JsonParticle,
    parser::{
        json_parsing_process::JsonParsingProcess,
        json_parsing_result::JsonParsingResult,
        steps::json_parsing_step::JsonParsingStep
    }
};

pub trait JsonParticleParser<T: JsonParticle>: Sized {
    type Step: JsonParsingStep<T, Self>;

    fn can_parse_string(&self, json: String) -> bool {
        self.can_parse(&JsonParsingProcess::new_for_json(json))
    }

    fn can_parse(&self, parsing_process: &JsonParsingProcess) -> bool;

    fn parse_string(self, json: String) -> JsonParsingResult<T> {
        self.parse(&mut JsonParsingProcess::new_for_json(json))
    }

    fn parse(mut self, parsing_process: &mut JsonParsingProcess) -> JsonParsingResult<T> {
        let step = self.get_step();
        let result = step.execute(&mut self, parsing_process);
        match result {
            Some(error) => JsonParsingResult::with_error(error),
            None => {
                match self.create() {
                    Some(v) => JsonParsingResult::with_value(v),
                    None => JsonParsingResult::with_error_to_create(
                        "An error occurred during instantiation.".to_string(),
                        parsing_process.clone()
                    ),
                }
            },
        }
    }

    fn get_step(&mut self) -> Self::Step;

    fn create(self) -> Option<T>;
}
