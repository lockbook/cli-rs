use std::fmt::Display;

/// parser complexities:
///
/// compound flags (-am)
///
/// optional flags
///
/// out of order flags
pub trait Input {
    fn parse(&mut self, token: &str) -> usize;
    fn display_name(&self) -> String;
    fn type_name(&self) -> InputType;
}

#[derive(Debug, PartialEq)]
pub enum InputType {
    Flag,
    Arg,
}

impl Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
