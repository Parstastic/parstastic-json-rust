use crate::parser::{
    json_parsing_process::JsonParsingProcess,
    json_parsing_result::JsonParsingResultError,
    steps::{
        json_parsing_step::JsonParsingStep,
        loop_step::LoopStep
    }
};

pub struct WhileLoopStep<T: JsonParsingStep, F: Fn(&JsonParsingProcess) -> bool> {
    instruction: T,
    continue_criteria: F
}

impl<T: JsonParsingStep, F: Fn(&JsonParsingProcess) -> bool> WhileLoopStep<T, F> {
    pub fn new(instruction: T, continue_criteria: F) -> Self {
        Self {
            instruction,
            continue_criteria
        }
    }
}

impl<T: JsonParsingStep, F: Fn(&JsonParsingProcess) -> bool> JsonParsingStep for WhileLoopStep<T, F> {
    fn execute(&self, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        while (self.continue_criteria)(parsing_process) {
            let result = self.instruction.execute(parsing_process);
            if result.is_some() {
                return result;
            }
        }
        None
    }
}

impl<T: JsonParsingStep, F: Fn(&JsonParsingProcess) -> bool> LoopStep for WhileLoopStep<T, F> {
    
}
