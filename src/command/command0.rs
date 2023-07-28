use crate::{input::Input, parser::Cmd};

use super::{command1::Command1, DocInfo, ParserInfo};

pub struct Command0<'a> {
    docs: DocInfo,

    subcommands: Vec<Box<dyn Cmd>>,
    handler: Option<Box<dyn FnMut() + 'a>>,
}

impl<'a> ParserInfo for Command0<'a> {
    fn symbols(&mut self) -> Vec<&mut dyn Input> {
        vec![]
    }

    fn call_handler(&mut self) {
        if let Some(handler) = &mut self.handler {
            handler()
        }
    }

    fn subcommands(&mut self) -> &mut Vec<Box<dyn Cmd>> {
        &mut self.subcommands
    }

    fn docs(&self) -> &DocInfo {
        &self.docs
    }

    fn push_parent(&mut self, _parents: &[String]) {
        panic!("command0 shouldn't have a parent pushed on to it");
    }
}

impl<'a> Command0<'a> {
    pub fn name(name: &str) -> Self {
        Self {
            docs: DocInfo {
                name: name.to_string(),
                ..Default::default()
            },
            subcommands: vec![],
            handler: None,
        }
    }

    pub fn description(mut self, description: &str) -> Self {
        self.docs.description = Some(description.to_string());
        self
    }

    pub fn input<T: Input>(self, input: T) -> Command1<'a, T> {
        Command1 {
            docs: self.docs,
            handler: None,
            in1: input,

            subcommands: self.subcommands,
        }
    }

    pub fn handler<F>(mut self, handler: F) -> Self
    where
        F: FnMut() + 'a,
    {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn subcommand<C: Cmd + 'static>(mut self, mut sub: C) -> Self {
        sub.push_parent(&self.docs.parents);
        self.subcommands.push(Box::new(sub));
        self
    }
}
