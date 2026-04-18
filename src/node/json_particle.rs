use crate::node::stringify_options::StringifyOptions;

pub trait JsonParticle {
    fn stringify(&self) -> String {
        self.stringify_with_options(StringifyOptions::default())
    }

    fn stringify_with_options(&self, options: StringifyOptions) -> String;
}
