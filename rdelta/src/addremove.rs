#[derive(Debug, PartialEq)]
pub enum AddRemove {
    Add(String),
    Same(String),
    Remove(String),
    Replace(String, String),
}
