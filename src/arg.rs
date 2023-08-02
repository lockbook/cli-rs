use std::str::FromStr;

use crate::{
    input::{Input, InputType},
    parser::{CliResult, ParseError},
};

#[derive(Default)]
pub struct Arg<T: FromStr> {
    pub name: String,
    pub description: Option<String>,
    pub value: Option<T>,
}

impl<T> Arg<T>
where
    T: FromStr,
{
    pub fn name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            value: None,
        }
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn get(&self) -> &T {
        self.value.as_ref().unwrap()
    }
}

impl Arg<String> {
    pub fn str(name: &str) -> Self {
        Self::name(name)
    }
}

impl Arg<i32> {
    pub fn i32(name: &str) -> Self {
        Self::name(name)
    }
}

impl<T: FromStr> Input for Arg<T> {
    fn parse(&mut self, token: &str) -> CliResult<usize> {
        if token.len() > 2 && &token[0..1] == "-" && &token[0..2] == "--" {
            eprintln!(
                "unexpected flag \"{}\" found while looking for argument \"{}\"",
                token, self.name
            );
            return Err(ParseError::UnexpectedToken);
        }
        self.value = Some(token.parse().map_err(|_| {
            eprintln!("{} cannot be parsed for {}.", token, self.name);
            ParseError::FromStrFailure
        })?);

        Ok(1)
    }

    fn display_name(&self) -> String {
        self.name.clone()
    }

    fn type_name(&self) -> InputType {
        InputType::Arg
    }
}
