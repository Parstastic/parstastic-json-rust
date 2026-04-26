use crate::{
    node::json_particle::JsonParticle,
    parser::{
        json_parsing_process::JsonParsingProcess,
        json_parsing_result::JsonParsingResultError,
        parsers::json_particle_parser::JsonParticleParser,
        steps::{
            export_step::ExportStep,
            json_parsing_step::JsonParsingStep,
        }
    }
};

pub struct OrStep<const N: usize, JP: JsonParticle, JPP: JsonParticleParser<JP>> {
    if_steps: [(Box<dyn Fn(&JPP, &JsonParsingProcess) -> bool>, Box<dyn JsonParsingStep<JP, JPP>>); N],
    else_step: Box<dyn JsonParsingStep<JP, JPP>>
}

impl<const N: usize, JP: JsonParticle + 'static,  JPP: JsonParticleParser<JP> + 'static> OrStep<N, JP, JPP> {
    pub fn new(
        if_steps: [(Box<dyn Fn(&JPP, &JsonParsingProcess) -> bool>, Box<dyn JsonParsingStep<JP, JPP>>); N],
        else_step: Box<dyn JsonParsingStep<JP, JPP>>
    ) -> Self {
        Self {
            if_steps,
            else_step
        }
    }

    pub fn else_error(if_steps: [(Box<dyn Fn(&JPP, &JsonParsingProcess) -> bool>, Box<dyn JsonParsingStep<JP, JPP>>); N]) -> Self {
        Self::new(
            if_steps,
            Box::new(ExportStep::new(|_, _| false))
        )
    }

    pub fn else_success(if_steps: [(Box<dyn Fn(&JPP, &JsonParsingProcess) -> bool>, Box<dyn JsonParsingStep<JP, JPP>>); N]) -> Self {
        Self::new(
            if_steps,
            Box::new(ExportStep::new(|_, _| true))
        )
    }
}

impl<const N: usize, JP: JsonParticle, JPP: JsonParticleParser<JP>> JsonParsingStep<JP, JPP> for OrStep<N, JP, JPP> {
    fn execute(&self, parser: &mut JPP, parsing_process: &mut JsonParsingProcess) -> Option<JsonParsingResultError> {
        for if_step in self.if_steps.iter() {
            if (if_step.0)(parser, parsing_process) {
                return if_step.1.execute(parser, parsing_process);
            }
        }
        self.else_step.execute(parser, parsing_process)
    }
}
