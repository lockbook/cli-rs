use super::{command4::Command4, DocInfo, ParserInfo};
use crate::{
    cli_error::{CliError, CliResult},
    input::Input,
    parser::Cmd,
};

type Callback3<'a, T1, T2, T3> = Box<dyn FnMut(&T1, &T2, &T3) -> CliResult<()> + 'a>;
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

    fn call_handler(&mut self) -> CliResult<()> {
        if let Some(handler) = &mut self.handler {
            handler(&self.in1, &self.in2, &self.in3)
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

impl<'a, T1: Input, T2: Input, T3: Input> Command3<'a, T1, T2, T3> {
    pub fn handler<F>(mut self, handler: F) -> Self
    where
        F: FnMut(&T1, &T2, &T3) -> CliResult<()> + 'a,
    {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn input<T4: Input>(self, in4: T4) -> Command4<'a, T1, T2, T3, T4> {
        Command4 {
            docs: self.docs,
            handler: None,

            in1: self.in1,
            in2: self.in2,
            in3: self.in3,
            in4,

            subcommands: self.subcommands,
        }
    }

    pub fn subcommand<C: Cmd + 'static>(mut self, mut sub: C) -> Self {
        sub.push_parent(&self.docs.parents);
        sub.push_parent(&[self.docs.name.clone()]);
        self.subcommands.push(Box::new(sub));
        self
    }
}
