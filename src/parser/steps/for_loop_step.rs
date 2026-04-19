use crate::parser::{
    json_parsing_process::JsonParsingProcess,
    json_parsing_result::JsonParsingResultError,
    steps::{
        json_parsing_step::JsonParsingStep,
        loop_step::LoopStep
    }
};

pub struct ForLoopStep<T: JsonParsingStep> {
    instruction: T,
    iterations: u32
}

impl<T: JsonParsingStep> ForLoopStep<T> {
    pub fn new(instruction: T, iterations: u32) -> Self {
        Self {
            instruction,
            iterations
        }
    }
}

impl<T: JsonParsingStep> JsonParsingStep for ForLoopStep<T> {
    fn execute(&self, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        for _ in 0..self.iterations {
            let result = self.instruction.execute(parsing_process);
            if result.is_some() {
                return result;
            }
        }
        None
    }
}

impl<T: JsonParsingStep> LoopStep for ForLoopStep<T> {
    
}
