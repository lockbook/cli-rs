use std::{env, process::exit};

use crate::input::Input;

pub type Command = Command0;

pub trait Cmd {
    fn name(&self) -> String;
    fn symbols(&mut self) -> Vec<&mut dyn Input>;
    fn subcommands(&mut self) -> &mut Vec<Box<dyn Cmd>>;
    fn call_handler(&mut self);

    // split this out into a trait that is pub, make the rest not pub
    fn parse(&mut self) {
        let args: Vec<String> = env::args().collect();
        self.parse_args(1, &args);
    }

    fn parse_args(&mut self, mut marker: usize, args: &[String]) {
        // try to match subcommands
        {
            let mut subcommand_index = None;
            let subcommands = self.subcommands();
            for i in 0..subcommands.len() {
                if subcommands[i].name() == args[marker] {
                    subcommand_index = Some(i);
                }
            }

            if let Some(index) = subcommand_index {
                let mut subcommand = subcommands.remove(index);
                marker += 1;
                return subcommand.parse_args(marker, args);
            }
        }

        // handle flags
        let mut symbols = self.symbols();
        if args.len() < symbols.len() {
            for i in args.len()..symbols.len() {
                println!(
                    "{} \"{}\" not provided",
                    symbols[i].type_name(),
                    symbols[i].display_name()
                )
            }

            exit(1);
        }

        for i in 0..symbols.len() {
            symbols[i].parse(args, i);
        }

        self.call_handler();
    }
}

pub struct Command0 {
    name: String,

    subcommands: Vec<Box<dyn Cmd>>,
    handler: Option<Box<dyn FnMut()>>,
}

impl Cmd for Command0 {
    fn symbols(&mut self) -> Vec<&mut dyn Input> {
        vec![]
    }

    fn call_handler(&mut self) {
        if let Some(handler) = &mut self.handler {
            handler()
        }
    }

    fn subcommands(&mut self) -> &mut Vec<Box<dyn Cmd>> {
        &mut self.subcommands
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Command0 {
    pub fn name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            subcommands: vec![],
            handler: None,
        }
    }

    pub fn input<T: Input>(self, input: T) -> Command1<T> {
        Command1 {
            name: self.name,
            handler: None,
            in1: input,

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

    pub fn subcommand<C: Cmd + 'static>(mut self, sub: C) -> Self {
        self.subcommands.push(Box::new(sub));
        self
    }
}

pub struct Command1<T1: Input> {
    name: String,

    subcommands: Vec<Box<dyn Cmd>>,
    handler: Option<Box<dyn FnMut(&T1)>>,

    in1: T1,
}

impl<T1> Cmd for Command1<T1>
where
    T1: Input,
{
    fn symbols(&mut self) -> Vec<&mut dyn Input> {
        vec![&mut self.in1]
    }

    fn call_handler(&mut self) {
        if let Some(handler) = &mut self.handler {
            handler(&self.in1);
        }
    }

    fn subcommands(&mut self) -> &mut Vec<Box<dyn Cmd>> {
        &mut self.subcommands
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl<T1: Input> Command1<T1> {
    pub fn input<T2: Input>(self, in2: T2) -> Command2<T1, T2> {
        Command2 {
            name: self.name,
            handler: None,

            in1: self.in1,
            in2,

            subcommands: self.subcommands,
        }
    }

    pub fn handler<F>(mut self, handler: F) -> Self
    where
        F: FnMut(&T1) + 'static,
    {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn subcommand<C: Cmd + 'static>(mut self, sub: C) -> Self {
        self.subcommands.push(Box::new(sub));
        self
    }
}

pub struct Command2<T1: Input, T2: Input> {
    name: String,

    subcommands: Vec<Box<dyn Cmd>>,
    handler: Option<Box<dyn FnMut(&T1, &T2)>>,

    in1: T1,
    in2: T2,
}

impl<T1, T2> Cmd for Command2<T1, T2>
where
    T1: Input,
    T2: Input,
{
    fn symbols(&mut self) -> Vec<&mut dyn Input> {
        vec![&mut self.in1, &mut self.in2]
    }

    fn call_handler(&mut self) {
        if let Some(handler) = &mut self.handler {
            handler(&self.in1, &self.in2);
        }
    }

    fn subcommands(&mut self) -> &mut Vec<Box<dyn Cmd>> {
        &mut self.subcommands
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl<T1: Input, T2: Input> Command2<T1, T2> {
    pub fn handler<F>(mut self, handler: F) -> Self
    where
        F: FnMut(&T1, &T2) + 'static,
    {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn subcommand<C: Cmd + 'static>(mut self, sub: C) -> Self {
        self.subcommands.push(Box::new(sub));
        self
    }
}
