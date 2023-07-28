// todo: args
// todo: long flags
// todo: short flags
// todo: help
// todo: completions
//
// should this describe what it wants up front, or just describe what happeend

use std::env;

use cli_rs::{arg::Arg, command::Command, parser::Cmd};

fn main() {
    // Command::name("Greeter")
    //     .input(Arg::<String>::new("name"))
    //     .handler(|name| println!("Hi {}", name.get()))
    //     .parse();

    println!("{:?}", env::args());
    Command::name("lockbook")
        .subcommand(
            Command::name("edit")
                .input(Arg::<String>::name("target"))
                .handler(|target| println!("editing target file: {}", target.get())),
        )
        .parse();
}
