use super::{DocInfo, ParserInfo};
use crate::{
    cli_error::{CliError, CliResult},
    input::Input,
    parser::Cmd,
};

type Callback4<'a, T1, T2, T3, T4> = Box<dyn FnMut(&T1, &T2, &T3, &T4) -> CliResult<()> + 'a>;
pub struct Command4<'a, T1: Input, T2: Input, T3: Input, T4: Input> {
    pub docs: DocInfo,

    pub subcommands: Vec<Box<dyn Cmd + 'a>>,
    pub handler: Option<Callback4<'a, T1, T2, T3, T4>>,

    pub in1: T1,
    pub in2: T2,
    pub in3: T3,
    pub in4: T4,
}

impl<'a, T1, T2, T3, T4> ParserInfo for Command4<'a, T1, T2, T3, T4>
where
    T1: Input,
    T2: Input,
    T3: Input,
    T4: Input,
{
    fn symbols(&mut self) -> Vec<&mut dyn Input> {
        vec![&mut self.in1, &mut self.in2, &mut self.in3, &mut self.in4]
    }

    fn call_handler(&mut self) -> CliResult<()> {
        if let Some(handler) = &mut self.handler {
            handler(&self.in1, &self.in2, &self.in3, &self.in4)
        } else {
            Err(CliError::from(format!(
                "No handler hooked up to {}",
                self.docs.cmd_path()
            )))
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

    fn complete_subcommand(&mut self, sub_idx: usize, tokens: &[String]) -> Result<(), CliError> {
        self.subcommands[sub_idx].complete_args(tokens)
    }

    fn parse_subcommand(&mut self, sub_idx: usize, tokens: &[String]) -> Result<(), CliError> {
        self.subcommands[sub_idx].parse_args(tokens)
    }
}

impl<'a, T1: Input, T2: Input, T3: Input, T4: Input> Command4<'a, T1, T2, T3, T4> {
    pub fn handler<F>(mut self, handler: F) -> Self
    where
        F: FnMut(&T1, &T2, &T3, &T4) -> CliResult<()> + 'a,
    {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn subcommand<C: Cmd + 'static>(mut self, mut sub: C) -> Self {
        sub.push_parent(&self.docs.parents);
        sub.push_parent(&[self.docs.name.clone()]);
        self.subcommands.push(Box::new(sub));
        self
    }
}
