use std::{env, fmt::Write};

use crate::{
    cli_error::{CliError, CliResult},
    command::{CompletionMode, ParserInfo},
    flag::Flag,
    input::{Input, InputType},
};

use colored::*;

impl<C> Cmd for C where C: ParserInfo {}

pub struct CompOut {
    pub name: String,
    pub desc: Option<String>,
}

fn version_flag() -> Flag<'static, bool> {
    Flag::bool("version").description("display CLI version")
}

fn help_flag() -> Flag<'static, bool> {
    Flag::bool("help").description("view help")
}

pub trait Cmd: ParserInfo {
    fn gen_help(&mut self) -> CliError {
        let cmd_path = self.docs().cmd_path();
        let mut help_message = String::new();

        write!(help_message, "{}", cmd_path.bold().green()).unwrap();

        if let Some(description) = &self.docs().description {
            write!(help_message, " - {description}").unwrap();
        }

        writeln!(help_message, "\n").unwrap();

        let mut version = version_flag();
        let mut help = help_flag();
        let mut built_in: Vec<&mut dyn Input> = vec![&mut help];
        if self.docs().version.is_some() {
            built_in.push(&mut version);
        }
        let subcommands = self.subcommand_docs();
        writeln!(help_message, "{}", "USAGE:".bold().yellow()).unwrap();
        if subcommands.is_empty() {
            let usage = format!("{cmd_path} [options], <args>").bold();
            writeln!(help_message, "\t{usage}").unwrap();
            writeln!(help_message, "\n{}", "FLAGS:".yellow().bold()).unwrap();

            let width = self
                .symbols()
                .into_iter()
                .map(|s| s.display_name().len() + 3)
                .chain([10]) // --version
                .max()
                .unwrap();

            for symbol in self.symbols().iter().chain(built_in.iter()) {
                if symbol.type_name() == InputType::Flag {
                    write!(help_message, "\t--{:width$}", symbol.display_name().bold()).unwrap();
                    if let Some(desc) = symbol.description() {
                        write!(help_message, " {desc}").unwrap();
                    }
                    writeln!(help_message).unwrap();
                }
            }
            writeln!(help_message, "\n{}", "ARGS:".yellow().bold()).unwrap();
            for symbol in self.symbols() {
                if symbol.type_name() == InputType::Arg {
                    write!(help_message, "\t{:width$}", symbol.display_name().bold()).unwrap();
                    if let Some(desc) = symbol.description() {
                        write!(help_message, " {desc}").unwrap();
                    }
                    writeln!(help_message).unwrap();
                }
            }
        } else {
            let usage = format! {"{cmd_path} <subcommand>"}.bold();
            writeln!(help_message, "\t{usage}").unwrap();
            writeln!(help_message, "\n{}", "SUBCOMMANDS:".yellow().bold()).unwrap();
            let sub_width = subcommands.iter().map(|s| s.name.len()).max().unwrap();
            for subcommand in subcommands {
                write!(help_message, "\t{:sub_width$}", subcommand.name.bold()).unwrap();
                if let Some(description) = subcommand.description {
                    write!(help_message, " {description}").unwrap();
                }

                writeln!(help_message).unwrap();
            }

            writeln!(help_message, "\n{}", "FLAGS:".yellow().bold()).unwrap();

            let flag_width = built_in
                .iter()
                .map(|s| s.display_name().len() + 3)
                .max()
                .unwrap();

            for symbol in built_in {
                if symbol.type_name() == InputType::Flag {
                    write!(
                        help_message,
                        "\t--{:flag_width$}",
                        symbol.display_name().bold()
                    )
                    .unwrap();
                    if let Some(desc) = symbol.description() {
                        write!(help_message, " {desc}").unwrap();
                    }
                    writeln!(help_message).unwrap();
                }
            }
        }

        CliError::from(help_message)
    }

    // split this out into a trait that is pub, make the rest not pub
    fn parse(&mut self) -> CliResult<()> {
        let args: Vec<String> = env::args().collect();
        // cmd complete shell word_idx [input]
        if args.len() >= 5 && args[1] == "complete" {
            let name = self.docs().name.to_string();
            // going to need some serious tests here
            let shell = args[2].parse::<CompletionMode>().unwrap();
            let prompt: Vec<String> = if shell == CompletionMode::Fish {
                let prompt = &args[4];
                prompt.split(' ').map(|s| s.to_string()).collect()
            } else {
                let idx: usize = args[3].parse().unwrap();
                let prompt = &args[4];
                prompt
                    .split(' ')
                    .map(|s| s.to_string())
                    .take(idx + 1)
                    .collect()
            };

            let mut last_command_location = 0;

            for (i, token) in prompt.iter().enumerate().rev() {
                if token == &name {
                    last_command_location = i;
                    break;
                }
            }

            let prompt = &prompt[last_command_location..];

            match self.complete_args(&prompt[1..]) {
                Ok(outputs) => {
                    match shell {
                        CompletionMode::Bash => {
                            for out in outputs {
                                println!("{}", out.name);
                            }
                        }
                        CompletionMode::Fish => {
                            for out in outputs {
                                if let Some(desc) = out.desc {
                                    println!("{}\t{}", out.name, desc);
                                } else {
                                    println!("{}", out.name);
                                }
                            }
                        }
                        CompletionMode::Zsh => {
                            let comps = outputs
                                .into_iter()
                                .map(|out| {
                                    if let Some(desc) = out.desc {
                                        let desc = desc.replace('\'', "");
                                        let desc = desc.replace('"', "");
                                        format!("'{}:{}'", out.name, desc)
                                    } else {
                                        format!("'{}'", out.name)
                                    }
                                })
                                .collect::<Vec<String>>()
                                .join(" ");

                            println!("_describe '{name}' \"({comps})\"");
                        }
                    };

                    return Ok(())
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }

        self.parse_args(&args[1..])
    }

    fn complete_args(&mut self, tokens: &[String]) -> CliResult<Vec<CompOut>> {
        let mut completions = vec![];
        if tokens.is_empty() {
            return Ok(completions);
        }

        let subcommands = self.subcommand_docs();

        // recurse into subcommand?
        if !subcommands.is_empty() && !tokens.is_empty() {
            let token = &tokens[0];
            let mut subcommand_index = None;
            for (idx, subcommand) in subcommands.iter().enumerate() {
                if &subcommand.name == token {
                    subcommand_index = Some(idx);
                }
            }

            // todo check this
            if let Some(index) = subcommand_index {
                return self.complete_subcommand(index, &tokens[1..]);
            }

            // print subcommands that begin with the token
            if tokens.len() == 1 && !tokens[0].starts_with('-') {
                for sub in subcommands {
                    if sub.name.starts_with(token) {
                        let name = &sub.name;
                        let desc = &sub.description;
                        completions.push(CompOut {
                            name: name.to_string(),
                            desc: desc.to_owned(),
                        })
                    }
                }

                return Ok(completions);
            }
        }

        let has_version = self.docs().version.is_some();
        let mut symbols = self.symbols();

        let mut positional_args_so_far = 0;
        if tokens.len() > 1 {
            for token in &tokens[0..tokens.len() - 1] {
                if !token.starts_with('-') {
                    // in a future where we manage errors more properly, this section could be
                    // closer to how the parser works, eliminating consumed symbols and helping
                    // the end user not see completions for flags they've already typed. Presently
                    // that code would start outputting errors.
                    positional_args_so_far += 1;
                }
            }
        }

        let token = &tokens[tokens.len() - 1];
        if let Some(mut completion_token) = token.strip_prefix('-') {
            if let Some(second_dash_removed) = completion_token.strip_prefix('-') {
                completion_token = second_dash_removed;
            }
            let value_completion = completion_token.split('=').collect::<Vec<&str>>();
            if value_completion.len() > 1 {
                for symbol in &mut symbols {
                    if symbol.display_name() == value_completion[0] {
                        for completion in symbol.complete(value_completion[1])? {
                            completions.push(CompOut {
                                name: format!("--{}={completion}", symbol.display_name()),
                                desc: None,
                            });
                        }
                        return Ok(completions);
                    }
                }
            }

            let mut version = version_flag();
            let mut help = help_flag();
            let mut built_in: Vec<&mut dyn Input> = vec![&mut help];
            if has_version {
                built_in.push(&mut version);
            }

            symbols
                .iter()
                .chain(built_in.iter())
                .filter(|sym| sym.type_name() == InputType::Flag)
                .filter(|sym| sym.display_name().starts_with(completion_token))
                .for_each(|flag| {
                    if flag.is_bool_flag() {
                        completions.push(CompOut {
                            name: format!("--{}", flag.display_name()),
                            desc: flag.description(),
                        });
                    } else {
                        completions.push(CompOut {
                            name: format!("--{}=", flag.display_name()),
                            desc: flag.description(),
                        });
                    }
                });
        } else {
            let arg = symbols
                .iter_mut()
                .filter(|sym| sym.type_name() == InputType::Arg)
                .nth(positional_args_so_far);

            if let Some(arg) = arg {
                for option in arg.complete(token)? {
                    completions.push(CompOut {
                        name: option.to_string(),
                        desc: None,
                    });
                }
            }
        }

        Ok(completions)
    }

    fn parse_args(&mut self, tokens: &[String]) -> CliResult<()> {
        let subcommands = self.subcommand_docs();
        let symbols = self.symbols();
        let required_args = symbols.iter().filter(|f| !f.has_default()).count();

        if tokens.is_empty() && (required_args > 0 || !subcommands.is_empty()) {
            return Err(self.gen_help());
        }

        // try to match subcommands
        if !tokens.is_empty() {
            let token = &tokens[0];
            if token == "--help" {
                println!("{}", self.gen_help().msg);
                return Ok(());
            }

            if token == "--version" {
                let docs = &self.docs();
                if let Some(version) = &docs.version {
                    println!("{} -- {}", docs.cmd_path(), version);
                    return Ok(());
                }
            }
            if !subcommands.is_empty() {
                for (idx, subcommand) in subcommands.iter().enumerate() {
                    if &subcommand.name == token {
                        return self.parse_subcommand(idx, &tokens[1..]);
                    }
                }

                return Err(CliError::from(format!("{token} is not a valid subcommand")));
            }
        }

        let mut symbols = self.symbols();

        for token in tokens {
            if token.starts_with('-') {
                let mut token_matched = false;
                for symbol in &mut symbols {
                    if !symbol.parsed() && symbol.type_name() == InputType::Flag {
                        let consumed = symbol.parse(token)?;
                        if consumed {
                            token_matched = true;
                        }
                    }
                }
                if !token_matched {
                    return Err(CliError::from(format!(
                        "Unexpected flag-like token found {token}"
                    )));
                }
            } else {
                'args: for symbol in &mut symbols {
                    if !symbol.parsed() && symbol.type_name() == InputType::Arg {
                        symbol.parse(token)?;
                        break 'args;
                    }
                }
            }
        }

        for symbol in symbols {
            if symbol.type_name() == InputType::Arg && !symbol.has_default() && !symbol.parsed() {
                return Err(CliError::from(format!(
                    "Missing required argument: {}",
                    symbol.display_name()
                )));
            }
        }

        self.call_handler()
    }
}
