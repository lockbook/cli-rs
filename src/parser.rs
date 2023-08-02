use std::{env, process::exit, str::FromStr};

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
        }
    }

    // split this out into a trait that is pub, make the rest not pub
    fn parse(&mut self) {
        let args: Vec<String> = env::args().collect();
        // cmd complete shell word_idx [input]
        if args.len() >= 4 {
            if args[1] == "complete" {
                let mut status = 0;
                let shell = args[2].parse::<CompletionMode>().unwrap();
                let idx = if shell == CompletionMode::Fish {
                    args[4..].len()
                } else {
                    args[3].parse().unwrap()
                };

                // chop off the `cmd complet shell idx` portion and truncate after the current word
                // index
                if args.len() > 4 + idx + 1 {
                    // going to need some serious tests here
                    status = match self.complete_args(&args[4..4 + idx + 1]) {
                        Ok(()) => 0,
                        Err(err) => err as i32,
                    };
                }
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

            if let Some(index) = subcommand_index {
                return self.complete_args(&tokens[1..]);
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

        let mut remaining_tokens = (0..tokens.len()).collect::<Vec<usize>>();
    }

    fn parse_args(&mut self, tokens: &[String]) -> CliResult<()> {
        let subcommands = self.subcommand_docs();
        let symbols = self.symbols();
        let symbols_count = symbols.len();

        if tokens.len() == 0 && (symbols_count > 0 || subcommands.len() > 0) {
            self.print_help();
            return Err(ParseError::HelpPrinted);
        }

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
            let mut subcommand_index = None;
            for (idx, subcommand) in subcommands.iter().enumerate() {
                if &subcommand.name == token {
                    subcommand_index = Some(idx);
                }
            }

            if let Some(index) = subcommand_index {
                return self.parse_subcommand(index, &tokens[1..]);
            }
        }

        let mut remaining_tokens = (0..tokens.len()).collect::<Vec<usize>>();

        let mut symbols = self.symbols();

        let flags = symbols
            .iter_mut()
            .filter(|symbol| symbol.type_name() == InputType::Flag);

        for flag in flags {
            for idx in remaining_tokens.clone() {
                let consumed = flag.parse(&tokens[idx])?;
                if consumed == 1 {
                    remaining_tokens.remove(idx);
                }
            }
        }

        for i in 0..symbols.len() {
            symbols[i].parse(&tokens[i])?;
        }

        let mut args = symbols
            .iter_mut()
            .filter(|symbol| symbol.type_name() == InputType::Arg);

        for idx in remaining_tokens {
            match args.next() {
                Some(arg) => {
                    arg.parse(&tokens[idx])?;
                }
                None => {
                    eprintln!("Unexpected token found \"{}\"", &tokens[idx]);
                    return Err(ParseError::UnexpectedToken);
                }
            }
        }

        self.call_handler();
        Ok(())
    }
}
