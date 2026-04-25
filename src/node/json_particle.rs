use crate::node::stringify_options::StringifyOptions;

pub trait JsonParticle {
    type Value;

    type BorrowedValue<'a>
        where Self: 'a;

    fn extract_value(self) -> Self::Value;

    fn get_value<'a>(&'a self) -> Self::BorrowedValue<'a>;

    fn stringify(&self) -> String {
        self.stringify_with_options(&StringifyOptions::default())
    }

    fn stringify_with_options(&self, options: &StringifyOptions) -> String;
}
