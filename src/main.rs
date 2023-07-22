// todo: args
// todo: long flags
// todo: short flags
// todo: help
// todo: completions
//
// should this describe what it wants up front, or just describe what happeend

use cli_rs::{
    arg::Arg,
    command::{Cmd, Command},
};

fn main() {
    Command::name("lockbook")
        .input(Arg::<String>::new("name"))
        .handler(|name| println!("Hi {}", name.get()))
        .parse();

    // Command::name("lockbook")
    //     .subcommand(
    //         Command::name("edit")
    //             .input(Arg::new("target"))
    //             .handler(|target: Arg<String>| println!("editing target file: {}", target.get())),
    //     )
    //     .parse();
}
