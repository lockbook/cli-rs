use std::{process::exit, str::FromStr};

use crate::{arg::Arg, flag::Flag};

/// parser complexities:
///
/// compound flags (-am)
///
/// optional flags
///
/// out of order flags
pub trait Input {
    fn parse(&mut self, args: &[String], offset: usize);
    fn name(&self) -> String;
    fn type_name(&self) -> String;
}

impl<T: FromStr> Input for Arg<T> {
    fn parse(&mut self, args: &[String], offset: usize) {
        self.value = Some(args[offset].parse().unwrap_or_else(|_| {
            println!("{} cannot be parsed for {}", args[offset], self.name);
            exit(1);
        }));
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn type_name(&self) -> String {
        "Arg".to_string()
    }
}

impl<T: FromStr> Input for Flag<T> {
    fn parse(&mut self, args: &[String], offset: usize) {
        todo!()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn type_name(&self) -> String {
        "Flag".to_string()
    }
}
