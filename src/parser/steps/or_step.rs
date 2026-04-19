use std::collections::HashMap;

use crate::parser::{
    json_parsing_process::JsonParsingProcess,
    json_parsing_result::JsonParsingResultError,
    steps::{
        export_step::ExportStep,
        json_parsing_step::JsonParsingStep
    }
};

pub struct OrStep {
    if_steps: HashMap<Box<dyn JsonParsingStep>, Box<dyn Fn(&JsonParsingProcess) -> bool>>,
    else_step: Box<dyn JsonParsingStep>
}

impl OrStep {
    pub fn new(
        if_steps: HashMap<Box<dyn JsonParsingStep>, Box<dyn Fn(&JsonParsingProcess) -> bool>>,
        else_step: Box<dyn JsonParsingStep>
    ) -> Self {
        Self {
            if_steps,
            else_step
        }
    }

    pub fn else_error(if_steps: HashMap<Box<dyn JsonParsingStep>, Box<dyn Fn(&JsonParsingProcess) -> bool>>) -> Self {
        Self::new(
            if_steps,
            Box::new(ExportStep::new(|| false))
        )
    }

    pub fn else_success(if_steps: HashMap<Box<dyn JsonParsingStep>, Box<dyn Fn(&JsonParsingProcess) -> bool>>) -> Self {
        Self::new(
            if_steps,
            Box::new(ExportStep::new(|| true))
        )
    }
}

impl JsonParsingStep for OrStep {
    fn execute(&self, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        for if_step in self.if_steps.iter() {
            if (if_step.1)(parsing_process) {
                return if_step.0.execute(parsing_process);
            }
        }
        self.else_step.execute(parsing_process)
    }
}
