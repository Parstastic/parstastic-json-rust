use crate::node::stringify_options::{
    DEFAULT_STRINGIFY_OPTIONS,
    StringifyOptions
};

pub trait JsonParticle {
    fn stringify(&self) -> String {
        self.stringify_with_options(DEFAULT_STRINGIFY_OPTIONS)
    }

    fn stringify_with_options(&self, options: StringifyOptions) -> String;
}
