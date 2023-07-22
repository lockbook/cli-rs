use crate::flag::Flag;

pub struct Subcommand {
    pub name: String,

    pub handler: Option<Box<dyn FnOnce()>>,
}

impl Subcommand {
    fn flag<T>(self, name: &str, ex_value: T) -> Subcommand2<Flag<T>> {
        todo!()
    }
}

pub struct Subcommand2<T> {
    pub name: String,

    pub in1: T,
}
