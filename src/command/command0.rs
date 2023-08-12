use crate::{
    arg::Arg,
    cli_error::{CliError, CliResult},
    input::Input,
    parser::Cmd,
};

use super::{command1::Command1, CompletionMode, DocInfo, ParserInfo};

pub struct Command0<'a> {
    docs: DocInfo,

    subcommands: Vec<Box<dyn Cmd + 'a>>,
    handler: Option<Box<dyn FnMut() -> CliResult<()> + 'a>>,
}

impl<'a> ParserInfo for Command0<'a> {
    fn symbols(&mut self) -> Vec<&mut dyn Input> {
        vec![]
    }

    fn call_handler(&mut self) -> CliResult<()> {
        if let Some(handler) = &mut self.handler {
            handler()
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

    fn parse_subcommand(&mut self, sub_idx: usize, tokens: &[String]) -> Result<(), CliError> {
        self.subcommands[sub_idx].parse_args(tokens)
    }

    fn complete_subcommand(&mut self, sub_idx: usize, tokens: &[String]) -> Result<(), CliError> {
        self.subcommands[sub_idx].complete_args(tokens)
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

    pub fn with_completions(self) -> Self {
        let name = self.docs.name.clone();

        self.subcommand(
            Self::name("completions")
                .description("generate completions for a given shell")
                .input(Arg::<CompletionMode>::name("shell"))
                .handler(move |shell| {
                    shell.get().print_completion(&name);
                    Ok(())
                }),
        )
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
        F: FnMut() -> CliResult<()> + 'a,
    {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn subcommand<C: Cmd + 'a>(mut self, mut sub: C) -> Self {
        sub.push_parent(&self.docs.parents);
        self.subcommands.push(Box::new(sub));
        self
    }
}
