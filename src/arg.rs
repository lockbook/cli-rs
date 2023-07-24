use std::{process::exit, str::FromStr};

use crate::input::Input;

#[derive(Default)]
pub struct Arg<T: FromStr> {
    pub name: String,
    pub value: Option<T>,
}

impl<T> Arg<T>
where
    T: FromStr,
{
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: None,
        }
    }

    pub fn get(&self) -> &T {
        &self.value.as_ref().unwrap()
    }
}

impl<T: FromStr> Input for Arg<T> {
    fn parse(&mut self, args: &[String], offset: usize) -> usize {
        if args[offset].len() > 2 && &args[offset][0..1] == "-" && &args[offset][0..2] == "--" {
            return 0;
        }
        self.value = Some(args[offset].parse().unwrap_or_else(|_| {
            println!("{} cannot be parsed for {}", args[offset], self.name);
            exit(1);
        }));

        1
    }

    fn display_name(&self) -> String {
        self.name.clone()
    }

    fn type_name(&self) -> String {
        "Arg".to_string()
    }
}
