use crate::parser::{
    json_parsing_process::JsonParsingProcess,
    json_parsing_result::JsonParsingResultError,
    steps::{
        json_parsing_step::JsonParsingStep,
        loop_step::LoopStep
    }
};

pub struct WhileLoopStep<T: JsonParsingStep> {
    instruction: T,
    continue_criteria: Box<dyn Fn(&JsonParsingProcess) -> bool>
}

impl<T: JsonParsingStep> WhileLoopStep<T> {
    pub fn new<F>(instruction: T, continue_criteria: F) -> Self
        where F: Fn(&JsonParsingProcess) -> bool + 'static
    {
        Self {
            instruction,
            continue_criteria: Box::new(continue_criteria)
        }
    }
}

impl<T: JsonParsingStep> JsonParsingStep for WhileLoopStep<T> {
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

impl<T: JsonParsingStep> LoopStep for WhileLoopStep<T> {
    
}
