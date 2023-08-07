use super::{command3::Command3, DocInfo, ParserInfo};
use crate::{
    input::Input,
    parser::{Cmd, ParseError},
};

type Callback2<'a, T1, T2> = Box<dyn FnMut(&T1, &T2) + 'a>;
pub struct Command2<'a, T1: Input, T2: Input> {
    pub docs: DocInfo,

    pub subcommands: Vec<Box<dyn Cmd + 'a>>,
    pub handler: Option<Callback2<'a, T1, T2>>,

    pub in1: T1,
    pub in2: T2,
}

impl<'a, T1, T2> ParserInfo for Command2<'a, T1, T2>
where
    T1: Input,
    T2: Input,
{
    fn symbols(&mut self) -> Vec<&mut dyn Input> {
        vec![&mut self.in1, &mut self.in2]
    }

    fn call_handler(&mut self) {
        if let Some(handler) = &mut self.handler {
            handler(&self.in1, &self.in2);
        }
    }

    fn subcommand_docs(&self) -> Vec<DocInfo> {
        self.subcommands.iter().map(|s| s.docs().clone()).collect()
    }

    fn docs(&self) -> &DocInfo {
        &self.docs
    }

    fn push_parent(&mut self, parents: &[String]) {
        self.docs.parents.extend_from_slice(parents);
    }

    fn complete_subcommand(&mut self, sub_idx: usize, tokens: &[String]) -> Result<(), ParseError> {
        self.subcommands[sub_idx].complete_args(tokens)
    }

    fn parse_subcommand(&mut self, sub_idx: usize, tokens: &[String]) -> Result<(), ParseError> {
        self.subcommands[sub_idx].parse_args(tokens)
    }
}

impl<'a, T1: Input, T2: Input> Command2<'a, T1, T2> {
    pub fn handler<F>(mut self, handler: F) -> Self
    where
        F: FnMut(&T1, &T2) + 'a,
    {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn input<T3: Input>(self, in3: T3) -> Command3<'a, T1, T2, T3> {
        Command3 {
            docs: self.docs,
            handler: None,

            in1: self.in1,
            in2: self.in2,
            in3,

            subcommands: self.subcommands,
        }
    }
    pub fn subcommand<C: Cmd + 'static>(mut self, sub: C) -> Self {
        self.subcommands.push(Box::new(sub));
        self
    }
}
