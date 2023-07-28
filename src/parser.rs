use std::{env, process::exit};

use crate::{command::ParserInfo, input::InputType};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    HelpPrinted = 1,
    MissingArg = 2,
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

        let subcommands = self.subcommands();
        if subcommands.is_empty() {
            println!("usage: {cmd_path} [options], <args>");
        } else {
            println!("usage: {cmd_path} <subcommands>");
        }
    }

    // split this out into a trait that is pub, make the rest not pub
    fn parse(&mut self) {
        let args: Vec<String> = env::args().collect();
        let status = match self.parse_args(&args[1..]) {
            Ok(()) => 0,
            Err(err) => err as i32,
        };

        exit(status);
    }

    fn parse_args(&mut self, tokens: &[String]) -> Result<(), ParseError> {
        let subcommand_count = self.subcommands().len();
        let symbols = self.symbols();
        let symbols_count = symbols.len();

        if tokens.len() == 0 && (symbols_count > 0 || subcommand_count > 0) {
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
        if subcommand_count > 0 {
            let token = &tokens[0];
            let mut subcommand_index = None;
            let subcommands = self.subcommands();
            for (idx, subcommand) in subcommands.iter().enumerate() {
                if &subcommand.docs().name == token {
                    subcommand_index = Some(idx);
                }
            }

            if let Some(index) = subcommand_index {
                let mut subcommand = subcommands.remove(index);
                return subcommand.parse_args(&tokens[1..]);
            }
        }

        let mut remaining_tokens = (0..tokens.len()).collect::<Vec<usize>>();

        let mut symbols = self.symbols();
        let flags = symbols
            .iter_mut()
            .filter(|symbol| symbol.type_name() == InputType::Flag);

        for flag in flags {
            for idx in remaining_tokens.clone() {
                let consumed = flag.parse(&tokens[idx]);
                if consumed == 1 {
                    remaining_tokens.remove(idx);
                }
            }
        }

        for i in 0..symbols.len() {
            symbols[i].parse(&tokens[i]);
        }

        let mut args = symbols
            .iter_mut()
            .filter(|symbol| symbol.type_name() == InputType::Arg);

        for idx in remaining_tokens {
            match args.next() {
                Some(arg) => {
                    arg.parse(&tokens[idx]);
                }
                None => {
                    eprintln!("Unexpected token found \"{}\"", &tokens[idx]);
                    exit(1);
                }
            }
        }

        self.call_handler();
        Ok(())
    }
}
