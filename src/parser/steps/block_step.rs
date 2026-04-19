use crate::parser::{
    json_parsing_process::JsonParsingProcess,
    json_parsing_result::JsonParsingResultError,
    steps::json_parsing_step::JsonParsingStep
};

pub struct BlockStep<const N: usize> {
    instructions: [Box<dyn JsonParsingStep>; N]
}

impl<const N: usize> BlockStep<N> {
    pub fn new(instructions: [Box<dyn JsonParsingStep>; N]) -> Self {
        Self {
            instructions
        }
    }
}

impl<const N: usize> JsonParsingStep for BlockStep<N> {
    fn execute(&self, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        for instruction in self.instructions.iter() {
            let result = instruction.execute(parsing_process);
            if result.is_some() {
                return result;
            }
        }
        None
    }
}
