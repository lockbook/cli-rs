use std::{process::exit, str::FromStr};

use crate::input::{Input, InputType};

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
        self.value.as_ref().unwrap()
    }
}

impl<T: FromStr> Input for Arg<T> {
    fn parse(&mut self, token: &str) -> usize {
        if token.len() > 2 && &token[0..1] == "-" && &token[0..2] == "--" {
            eprintln!(
                "unexpected flag found {} while looking for argument {}",
                token, self.name
            );
            exit(1);
        }
        self.value = Some(token.parse().unwrap_or_else(|_| {
            eprintln!("{} cannot be parsed for {}", token, self.name);
            exit(1);
        }));

        1
    }

    fn display_name(&self) -> String {
        self.name.clone()
    }

    fn type_name(&self) -> InputType {
        InputType::Arg
    }
}
