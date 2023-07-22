#[derive(Default)]
pub struct Arg<T> {
    name: String,
    value: Option<T>,
}

impl<T> Arg<T> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: None,
        }
    }
}
