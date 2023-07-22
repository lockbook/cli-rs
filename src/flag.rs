// todo for bool it can just be existence
// todo what is the deal with short flags?
pub struct Flag<T> {
    pub(crate) short: bool,
    pub(crate) name: String,
    pub(crate) value: Option<T>,
}

impl<T> Flag<T> {
    pub fn long(name: &str, value: T) -> Self {
        if name.is_empty() {
            panic!("empty flag name provided");
        }

        // let short = name
        //     .chars()
        //     .next()
        //     .unwrap()
        //     .to_ascii_uppercase()
        //     .to_string();

        let name = name.to_lowercase();

        Self {
            short: false,
            name,
            value: Some(value),
        }
    }
}
