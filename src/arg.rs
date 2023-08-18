use std::str::FromStr;

use crate::{
    cli_error::{CliError, CliResult},
    input::{Completor, Input, InputType},
};

#[derive(Default)]
pub struct Arg<'a, T: FromStr + Clone> {
    pub name: String,
    pub description: Option<String>,
    pub value: Option<T>,
    pub completor: Option<Completor<'a>>,
    pub default_value: Option<T>,
}

impl<'a, T> Arg<'a, T>
where
    T: FromStr + Clone,
{
    pub fn name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            value: None,
            completor: None,
            default_value: None,
        }
    }

    pub fn default(mut self, default: T) -> Self {
        self.default_value = Some(default);
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn get(&self) -> T {
        self.value
            .clone()
            .unwrap_or_else(|| self.default_value.clone().unwrap())
    }

    pub fn completor<F>(mut self, completor: F) -> Self
    where
        F: FnMut(&str) -> CliResult<Vec<String>> + 'a,
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

impl<'a, T: FromStr + Clone> Input for Arg<'a, T> {
    fn parse(&mut self, token: &str) -> CliResult<bool> {
        if token.len() > 2 && &token[0..1] == "-" && &token[0..2] == "--" {
            return Err(CliError::from(format!(
                "unexpected flag \"{}\" found while looking for argument \"{}\"",
                token, self.name
            )));
        }
        self.value = Some(token.parse().map_err(|_| {
            CliError::from(format!("{} cannot be parsed for {}.", token, self.name))
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

    fn complete(&mut self, value: &str) -> CliResult<Vec<String>> {
        if let Some(completor) = &mut self.completor {
            completor(value)
        } else {
            Ok(vec![])
        }
    }

    fn is_bool_flag(&self) -> bool {
        false
    }

    fn description(&self) -> Option<String> {
        self.description.clone()
    }

    fn has_default(&self) -> bool {
        self.default_value.is_some()
    }
}
