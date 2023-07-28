use self::command0::Command0;
use crate::input::Input;
use crate::parser::Cmd;
use std::fmt::Write;

pub mod command0;
pub mod command1;
pub mod command2;
pub mod command3;

pub type Command<'a> = Command0<'a>;

pub trait ParserInfo {
    fn docs(&self) -> &DocInfo;
    fn symbols(&mut self) -> Vec<&mut dyn Input>;
    fn subcommands(&mut self) -> &mut Vec<Box<dyn Cmd>>;
    fn call_handler(&mut self);
    fn push_parent(&mut self, parents: &[String]);
}

#[derive(Default)]
pub struct DocInfo {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) parents: Vec<String>,
}

impl DocInfo {
    pub fn cmd_path(&self) -> String {
        let mut path = String::new();
        for parent in &self.parents {
            write!(path, "{parent} ").unwrap();
        }

        write!(path, "{}", self.name).unwrap();
        path
    }
}
