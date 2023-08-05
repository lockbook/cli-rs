use std::str::FromStr;

use crate::{
    input::{Input, InputType},
    parser::{CliResult, ParseError},
};

// todo existence
// todo short flags with a space
// todo short flag-sets
pub struct Flag<'a, T: Default + FromStr> {
    pub name: String,
    pub value: Option<T>,
    pub bool_flag: bool,
    pub completor: Option<Box<dyn FnMut(String) -> Vec<String> + 'a>>,
}

impl<'a> Flag<'a, bool> {
    pub fn bool(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: None,
            bool_flag: true,
            completor: None,
        }
    }
}

impl<'a, T: FromStr + Default> Flag<'a, T> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: None,
            bool_flag: false,
            completor: None,
        }
    }

    pub fn get(&self) -> &T {
        self.value.as_ref().unwrap()
    }

    pub fn completor<F>(mut self, completor: F) -> Self
    where
        F: FnMut(String) -> Vec<String> + 'a,
    {
        self.completor = Some(Box::new(completor));
        self
    }
}

impl<'a, T: FromStr + Default> Input for Flag<'a, T> {
    // for short flags with a space
    // should probably return a Result<bool, ParseError>
    fn parse(&mut self, token: &str) -> CliResult<bool> {
        // just handle the short flag first
        if token.len() == 2 && self.bool_flag {
            if &token[0..1] != "-" {
                // todo should this be an error?
                return Ok(false);
            }

            if token[1..2].to_uppercase() == self.name[0..1].to_uppercase() {
                self.value = Some("true".parse().unwrap_or_else(|_| unreachable!()));
                return Ok(true);
            }
        }

        // make it safe to index anywhere
        let min_size = self.name.len() + 2; // --
        if token.len() < min_size {
            return Ok(false);
        }

        // name check
        let flag_length = self.name.len();
        if token[2..flag_length + 2] != self.name {
            return Ok(false);
        }

        // extract value
        if let Some(eq_idx) = token.find('=') {
            let value = &token[eq_idx + 1..].to_string();

            self.value = Some(value.parse().map_err(|_| {
                eprintln!("{} cannot be parsed for {}", value, self.name);
                ParseError::FromStrFailure
            })?);

            return Ok(true);
        }

        if self.bool_flag {
            self.value = Some("true".parse().unwrap_or_else(|_| unreachable!()));
            return Ok(true);
        }

        Ok(false)
    }

    fn display_name(&self) -> String {
        self.name.clone()
    }

    fn type_name(&self) -> InputType {
        InputType::Flag
    }

    fn parsed(&self) -> bool {
        self.value.is_some()
    }

    fn complete(&mut self, value: &str) -> Vec<String> {
        todo!()
    }
}
