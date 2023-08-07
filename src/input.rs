use std::fmt::Display;

use crate::parser::CliResult;

/// parser complexities:
///
/// compound flags (-am)
///
/// optional flags
///
/// out of order flags
pub trait Input {
    fn parsed(&self) -> bool;
    fn parse(&mut self, token: &str) -> CliResult<bool>;
    fn display_name(&self) -> String;
    fn description(&self) -> Option<String>;
    fn type_name(&self) -> InputType;
    fn is_bool_flag(&self) -> bool;
    fn complete(&mut self, value: &str) -> Vec<String>;
}

#[derive(Debug, PartialEq)]
pub enum InputType {
    Flag,
    Arg,
}

pub type Completor<'a> = Box<dyn FnMut(&str) -> Vec<String> + 'a>;

impl Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
