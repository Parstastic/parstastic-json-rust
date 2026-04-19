use crate::parser::{
    json_parsing_process::JsonParsingProcess,
    json_parsing_result::JsonParsingResultError,
    steps::json_parsing_step::JsonParsingStep
};

pub struct StepCreationStep<T: JsonParsingStep> {
    step_creator: Box<dyn Fn() -> T>
}

impl<T: JsonParsingStep> StepCreationStep<T> {
    pub fn new<F>(step_creator: F) -> Self
        where F: Fn() -> T + 'static
    {
        Self {
            step_creator: Box::new(step_creator)
        }
    }
}

impl<T: JsonParsingStep> JsonParsingStep for StepCreationStep<T> {
    fn execute(&self, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        (self.step_creator)().execute(parsing_process)
    }
}
