use std::{env, process::exit};

use crate::{
    command::{CompletionMode, ParserInfo},
    input::InputType,
};

pub type CliResult<T> = Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    HelpPrinted = 1,
    MissingArg = 2,
    UnexpectedToken = 3,
    FromStrFailure = 4,
}

impl<C> Cmd for C where C: ParserInfo {}

pub trait Cmd: ParserInfo {
    fn print_help(&mut self) {
        let cmd_path = self.docs().cmd_path();

        print!("{cmd_path}");

        if let Some(description) = &self.docs().description {
            print!(" - {description}");
        }

        println!("\n");

        let subcommands = self.subcommand_docs();
        if subcommands.is_empty() {
            println!("usage: {cmd_path} [options], <args>");
        } else {
            println!("usage: {cmd_path} <subcommands>");
            println!("\nsubcommands:");
            for subcommand in subcommands {
                print!("{}", subcommand.name);
                if let Some(description) = subcommand.description {
                    print!(": {description}");
                }

                println!();
            }
        }
    }

    // split this out into a trait that is pub, make the rest not pub
    fn parse(&mut self) {
        let args: Vec<String> = env::args().collect();
        // cmd complete shell word_idx [input]
        if args.len() >= 5 {
            if args[1] == "complete" {
                // going to need some serious tests here
                let mut status = 0;
                let shell = args[2].parse::<CompletionMode>().unwrap();
                let prompt: Vec<String> = if shell == CompletionMode::Fish {
                    let prompt = &args[4];
                    prompt.split(" ").map(|s| s.to_string()).collect()
                } else {
                    let idx = args[3].parse().unwrap();
                    let prompt = &args[4];
                    prompt.split(" ").map(|s| s.to_string()).take(idx).collect()
                };

                status = match self.complete_args(&prompt[1..]) {
                    Ok(()) => 0,
                    Err(err) => err as i32,
                };
                exit(status);
            }
        }

        let status = match self.parse_args(&args[1..]) {
            Ok(()) => 0,
            Err(err) => err as i32,
        };
        exit(status);
    }

    fn complete_args(&mut self, tokens: &[String]) -> CliResult<()> {
        let subcommands = self.subcommand_docs();

        // recurse into subcommand?
        if subcommands.len() > 0 && tokens.len() > 0 {
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
            return Ok(());
        }

        // try to complete a subcommand
        if subcommands.len() > 0 && tokens.len() == 0 {
            for subcommand in subcommands {
                println!("{}", subcommand.name);
            }
            return Ok(());
        }
        let mut symbols = self.symbols();

        let mut positional_args_so_far = 0;
        if tokens.len() > 1 {
            for token in &tokens[0..tokens.len() - 1] {
                if !token.starts_with("-") {
                    // in a future where we manage errors more properly, this section could be
                    // closer to how the parser works, eliminating consumed symbols and helping
                    // the end user not see completions for flags they've already typed. Presently
                    // that code would start outputting errors.
                    positional_args_so_far += 1;
                }
            }
        }

        if tokens.len() == 0 {}

        Ok(())
    }

    fn parse_args(&mut self, tokens: &[String]) -> CliResult<()> {
        let subcommands = self.subcommand_docs();
        let symbols = self.symbols();
        let symbols_count = symbols.len();

        if tokens.len() == 0 && (symbols_count > 0 || subcommands.len() > 0) {
            self.print_help();
            return Err(ParseError::HelpPrinted);
        }

        // todo check this
        if tokens.len() < symbols.len() {
            for symbol in symbols.iter().skip(tokens.len()) {
                println!(
                    "{} \"{}\" not provided",
                    symbol.type_name(),
                    symbol.display_name()
                );
            }

            return Err(ParseError::MissingArg);
        }

        // try to match subcommands
        if subcommands.len() > 0 {
            let token = &tokens[0];
            for (idx, subcommand) in subcommands.iter().enumerate() {
                if &subcommand.name == token {
                    return self.parse_subcommand(idx, &tokens[1..]);
                }
            }
        }

        let mut symbols = self.symbols();

        for token in tokens {
            if token.starts_with("-") {
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
                    eprintln!("Unexpected flag-like token found {token}");
                    return Err(ParseError::UnexpectedToken);
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

        self.call_handler();
        Ok(())
    }
}
