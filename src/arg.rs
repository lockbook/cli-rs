use std::str::FromStr;

use crate::{
    input::{Input, InputType},
    parser::{CliResult, ParseError},
};

#[derive(Default)]
pub struct Arg<'a, T: FromStr> {
    pub name: String,
    pub description: Option<String>,
    pub value: Option<T>,
    pub completor: Option<Box<dyn FnMut(&str) -> Vec<String> + 'a>>,
}

impl<'a, T> Arg<'a, T>
where
    T: FromStr,
{
    pub fn name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            value: None,
            completor: None,
        }
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn get(&self) -> &T {
        self.value.as_ref().unwrap()
    }

    pub fn completor<F>(mut self, completor: F) -> Self
    where
        F: FnMut(&str) -> Vec<String> + 'a,
    {
        self.completor = Some(Box::new(completor));
        self
    }
}

impl<'a> Arg<'a, String> {
    pub fn str(name: &str) -> Self {
        Self::name(name)
    }
}

impl<'a> Arg<'a, i32> {
    pub fn i32(name: &str) -> Self {
        Self::name(name)
    }
}

impl<'a, T: FromStr> Input for Arg<'a, T> {
    fn parse(&mut self, token: &str) -> CliResult<bool> {
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

        Ok(true)
    }

    fn display_name(&self) -> String {
        self.name.clone()
    }

    fn type_name(&self) -> InputType {
        InputType::Arg
    }

    fn parsed(&self) -> bool {
        self.value.is_some()
    }

    fn complete(&mut self, value: &str) -> Vec<String> {
        if let Some(completor) = &mut self.completor {
            completor(value)
        } else {
            vec![]
        }
    }
}
