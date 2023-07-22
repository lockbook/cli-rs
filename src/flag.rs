use std::str::FromStr;

// todo for bool it can just be existence
// todo what is the deal with short flags?
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
