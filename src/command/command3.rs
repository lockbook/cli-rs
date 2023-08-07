use super::{DocInfo, ParserInfo};
use crate::{
    input::Input,
    parser::{Cmd, ParseError},
};

type Callback3<'a, T1, T2, T3> = Box<dyn FnMut(&T1, &T2, &T3) + 'a>;
pub struct Command3<'a, T1: Input, T2: Input, T3: Input> {
    pub docs: DocInfo,

    pub subcommands: Vec<Box<dyn Cmd + 'a>>,
    pub handler: Option<Callback3<'a, T1, T2, T3>>,

    pub in1: T1,
    pub in2: T2,
    pub in3: T3,
}

impl<'a, T1, T2, T3> ParserInfo for Command3<'a, T1, T2, T3>
where
    T1: Input,
    T2: Input,
    T3: Input,
{
    fn symbols(&mut self) -> Vec<&mut dyn Input> {
        vec![&mut self.in1, &mut self.in2, &mut self.in3]
    }

    fn call_handler(&mut self) {
        if let Some(handler) = &mut self.handler {
            handler(&self.in1, &self.in2, &self.in3);
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

impl<'a, T1: Input, T2: Input, T3: Input> Command3<'a, T1, T2, T3> {
    pub fn handler<F>(mut self, handler: F) -> Self
    where
        F: FnMut(&T1, &T2, &T3) + 'a,
    {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn subcommand<C: Cmd + 'static>(mut self, sub: C) -> Self {
        self.subcommands.push(Box::new(sub));
        self
    }
}
