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
    SubcommandNotFound = 3,
    UnexpectedToken = 4,
    FromStrFailure = 5,
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
                let shell = args[2].parse::<CompletionMode>().unwrap();
                let prompt: Vec<String> = if shell == CompletionMode::Fish {
                    let prompt = &args[4];
                    prompt.split(" ").map(|s| s.to_string()).collect()
                } else {
                    let idx: usize = args[3].parse().unwrap();
                    let prompt = &args[4];
                    prompt
                        .split(" ")
                        .map(|s| s.to_string())
                        .take(idx + 1)
                        .collect()
                };

                let status = match self.complete_args(&prompt[1..]) {
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

        let token = &tokens[tokens.len() - 1];
        if token.starts_with("-") {
            let completion_token: &str;
            if token.starts_with("--") {
                completion_token = &token[2..];
            } else {
                completion_token = &token[1..];
            }
            let value_completion = completion_token.split('=').collect::<Vec<&str>>();
            if value_completion.len() > 1 {
                for symbol in &mut symbols {
                    if symbol.display_name() == value_completion[0] {
                        for completion in symbol.complete(value_completion[1]) {
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
                .skip(positional_args_so_far)
                .next();

            if let Some(arg) = arg {
                for option in arg.complete(token) {
                    println!("{option}");
                }
            }
        }

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

        // try to match subcommands
        if subcommands.len() > 0 {
            let token = &tokens[0];
            for (idx, subcommand) in subcommands.iter().enumerate() {
                if &subcommand.name == token {
                    return self.parse_subcommand(idx, &tokens[1..]);
                }
            }

            eprintln!("{token} is not a valid subcommand");
            return Err(ParseError::SubcommandNotFound);
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

        for symbol in symbols {
            if symbol.type_name() == InputType::Arg && !symbol.parsed() {
                eprintln!("Missing required argument: {}", symbol.display_name());
                return Err(ParseError::MissingArg);
            }
        }

        self.call_handler();
        Ok(())
    }
}
