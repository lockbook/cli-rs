use crate::{input::Input, parser::Cmd};

use super::{command2::Command2, DocInfo, ParserInfo};

type Callback1<'a, T1> = Box<dyn FnMut(&T1) + 'a>;
pub struct Command1<'a, T1: Input> {
    pub docs: DocInfo,

    pub subcommands: Vec<Box<dyn Cmd>>,
    pub handler: Option<Callback1<'a, T1>>,

    pub in1: T1,
}

impl<'a, T1> ParserInfo for Command1<'a, T1>
where
    T1: Input,
{
    fn symbols(&mut self) -> Vec<&mut dyn Input> {
        vec![&mut self.in1]
    }

    fn call_handler(&mut self) {
        if let Some(handler) = &mut self.handler {
            handler(&self.in1);
        }
    }

    fn subcommands(&mut self) -> &mut Vec<Box<dyn Cmd>> {
        &mut self.subcommands
    }

    fn docs(&self) -> &DocInfo {
        &self.docs
    }

    fn push_parent(&mut self, parents: &[String]) {
        self.docs.parents.extend_from_slice(parents);
    }
}

impl<'a, T1: Input> Command1<'a, T1> {
    pub fn input<T2: Input>(self, in2: T2) -> Command2<'a, T1, T2> {
        Command2 {
            docs: self.docs,
            handler: None,

            in1: self.in1,
            in2,

            subcommands: self.subcommands,
        }
    }

    pub fn handler<F>(mut self, handler: F) -> Self
    where
        F: FnMut(&T1) + 'a,
    {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn subcommand<C: Cmd + 'static>(mut self, sub: C) -> Self {
        self.subcommands.push(Box::new(sub));
        self
    }
}
