use std::str::FromStr;

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

    pub fn get(self) -> T {
        self.value.unwrap()
    }
}
