use std::{process::exit, str::FromStr};

use crate::input::Input;

// todo existence
// todo short flags with a space
// todo short flag-sets
pub struct Flag<T: FromStr> {
    pub name: String,
    pub value: Option<T>,
}

impl Flag<bool> {
    pub fn bool(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: None,
        }
    }
}

impl<T: FromStr> Flag<T> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: None,
        }
    }
}

impl<T: FromStr> Input for Flag<T> {
    fn parse(&mut self, args: &[String], offset: usize) -> usize {
        let token = &args[offset];

        // make it safe to index anywhere
        let min_size = self.name.len() + 4; // --, =, and the value
        if token.len() < min_size {
            return 0;
        }

        // name check
        let flag_length = self.name.len();
        if token[2..flag_length + 2] != self.name {
            return 0;
        }

        // extract value
        if let Some(eq_idx) = token.find('=') {
            let value = &token[eq_idx..].to_string();

            self.value = Some(value.parse().unwrap_or_else(|_| {
                println!("{} cannot be parsed for {}", args[offset], self.name);
                exit(1);
            }));

            return 1;
        }

        0
    }

    fn display_name(&self) -> String {
        self.name.clone()
    }

    fn type_name(&self) -> String {
        "Flag".to_string()
    }
}
