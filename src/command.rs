use crate::{arg::Arg, flag::Flag, input::Input, subcommand::Subcommand};

pub struct Command {
    name: String,

    subcommands: Vec<Subcommand>,
    handler: Option<Box<dyn FnOnce()>>,
}

impl Command {
    pub fn name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            subcommands: vec![],
            handler: None,
        }
    }

    pub fn input<T: Input>(self, input: T) -> Command2<T> {
        Command2 {
            name: self.name,
            handler: None,
            in1: input,

            subcommands: self.subcommands,
        }
    }

    pub fn long_flag<T>(self, name: &str, ex_val: T) -> Command2<Flag<T>> {
        Command2 {
            name: self.name,
            handler: None,
            in1: Flag::long(name, ex_val),

            subcommands: self.subcommands,
        }
    }

    pub fn arg<T>(self, name: &str, ex_val: T) -> Command2<Arg<T>> {
        Command2 {
            name: self.name,
            handler: None,
            in1: Arg::new(name),

            subcommands: self.subcommands,
        }
    }

    pub fn handler<F>(mut self, handler: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn subcommand(mut self, sub: Subcommand) -> Self {
        self.subcommands.push(sub);
        self
    }
}

pub struct Command2<T1: Input> {
    name: String,

    subcommands: Vec<Subcommand>,
    handler: Option<Box<dyn FnOnce(T1)>>,

    in1: T1,
}

impl<T1: Input> Command2<T1> {
    pub fn long_flag<T>(self, name: &str, ex_val: T) -> Command2<Flag<T>> {
        Command2 {
            name: self.name,
            handler: None,
            in1: Flag::long(name, ex_val),
            subcommands: self.subcommands,
        }
    }

    pub fn arg<T>(self, name: &str, ex_val: T) -> Command2<Arg<T>> {
        Command2 {
            name: self.name,
            handler: None,
            in1: Arg::new(name),
            subcommands: self.subcommands,
        }
    }

    pub fn handler<F>(mut self, handler: F) -> Self
    where
        F: FnMut(T1) + 'static,
    {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn subcommand(mut self, sub: Subcommand) -> Self {
        self.subcommands.push(sub);
        self
    }
}

pub struct Command3<T1: Input, T2: Input> {
    name: String,

    subcommands: Vec<Subcommand>,
    handler: Option<Box<dyn FnOnce(T1)>>,

    in1: T1,
    in2: T2,
}

impl<T1: Input, T2: Input> Command3<T1, T2> {
    pub fn long_flag<T>(self, name: &str, ex_val: T) -> Command2<Flag<T>> {
        Command2 {
            name: self.name,
            handler: None,
            in1: Flag::long(name, ex_val),
            subcommands: self.subcommands,
        }
    }

    pub fn arg<T>(self, name: &str, ex_val: T) -> Command2<Arg<T>> {
        Command2 {
            name: self.name,
            handler: None,
            in1: Arg::new(name),
            subcommands: self.subcommands,
        }
    }

    pub fn handler<F>(mut self, handler: F) -> Self
    where
        F: FnMut(T1) + 'static,
    {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn subcommand(mut self, sub: Subcommand) -> Self {
        self.subcommands.push(sub);
        self
    }
}
