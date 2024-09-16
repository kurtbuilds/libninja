pub mod command;
pub mod extractor;

pub fn default<T: Default>() -> T {
    Default::default()
}
