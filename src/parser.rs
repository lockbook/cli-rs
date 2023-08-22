use std::{env, fmt::Write};

use crate::{
    cli_error::{CliError, CliResult, Exit},
    command::{CompletionMode, ParserInfo},
    input::InputType,
};

use colored::*;

impl<C> Cmd for C where C: ParserInfo {}

pub trait Cmd: ParserInfo {
    fn print_help(&mut self) -> CliError {
        let cmd_path = self.docs().cmd_path();
        let mut help_message = String::new();

        write!(help_message, "{}", cmd_path.bold().green()).unwrap();

        if let Some(description) = &self.docs().description {
            write!(help_message, " - {description}").unwrap();
        }

        writeln!(help_message, "\n").unwrap();

        let subcommands = self.subcommand_docs();
        writeln!(help_message, "{}", "USAGE:".bold().yellow()).unwrap();
        if subcommands.is_empty() {
            let usage = format!("{cmd_path} [options], <args>").bold();
            writeln!(help_message, "\t{usage}").unwrap();
            writeln!(help_message, "\n{}", "FLAGS:".yellow().bold()).unwrap();
            for symbol in self.symbols() {
                if symbol.type_name() == InputType::Flag {
                    write!(help_message, "\t--{}", symbol.display_name().bold()).unwrap();
                    if let Some(desc) = symbol.description() {
                        write!(help_message, ": {desc}").unwrap();
                    }
                    writeln!(help_message).unwrap();
                }
            }
            writeln!(help_message, "\n{}", "ARGS:".yellow().bold()).unwrap();
            for symbol in self.symbols() {
                if symbol.type_name() == InputType::Arg {
                    write!(help_message, "\t{}", symbol.display_name().bold()).unwrap();
                    if let Some(desc) = symbol.description() {
                        write!(help_message, ": {desc}").unwrap();
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
        }

        CliError::from(help_message)
    }

    // split this out into a trait that is pub, make the rest not pub
    fn parse(&mut self) {
        let args: Vec<String> = env::args().collect();
        // cmd complete shell word_idx [input]
        if args.len() >= 5 && args[1] == "complete" {
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

            self.complete_args(&prompt[1..]).exit_silently();
        }

        self.parse_args(&args[1..]).exit();
    }

    fn complete_args(&mut self, tokens: &[String]) -> CliResult<()> {
        if tokens.is_empty() {
            return Ok(());
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
            if tokens.len() == 1 {
                subcommands
                    .iter()
                    .filter_map(|sub| {
                        if sub.name.starts_with(token) {
                            Some(sub.name.to_string())
                        } else {
                            None
                        }
                    })
                    .for_each(|sub| println!("{sub}"));
            }
            return Ok(());
        }

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
                            println!("--{}={completion}", symbol.display_name());
                        }
                        return Ok(());
                    }
                }
            }

            symbols
                .iter()
                .filter(|sym| sym.type_name() == InputType::Flag)
                .filter(|sym| sym.display_name().starts_with(completion_token))
                .for_each(|flag| {
                    if flag.is_bool_flag() {
                        println!("--{}", flag.display_name());
                    } else {
                        println!("--{}=", flag.display_name());
                    }
                });
            // todo an interesting thing to explore later:
            // println!(r#" _describe 'command' "('-cmd1:description1' '-cmd2:description2')" "#);
        } else {
            let arg = symbols
                .iter_mut()
                .filter(|sym| sym.type_name() == InputType::Arg)
                .nth(positional_args_so_far);

            if let Some(arg) = arg {
                for option in arg.complete(token)? {
                    println!("{option}");
                }
            }
        }

        Ok(())
    }

    fn parse_args(&mut self, tokens: &[String]) -> CliResult<()> {
        let subcommands = self.subcommand_docs();
        let symbols = self.symbols();
        let required_args = symbols.iter().filter(|f| !f.has_default()).count();

        if tokens.is_empty() && (required_args > 0 || !subcommands.is_empty()) {
            return Err(self.print_help());
        }

        // try to match subcommands
        if !subcommands.is_empty() {
            let token = &tokens[0];
            for (idx, subcommand) in subcommands.iter().enumerate() {
                if &subcommand.name == token {
                    return self.parse_subcommand(idx, &tokens[1..]);
                }
            }

            return Err(CliError::from(format!("{token} is not a valid subcommand")));
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
