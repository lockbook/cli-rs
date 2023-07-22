// todo: args
// todo: long flags
// todo: short flags
// todo: help
// todo: completions
//
// should this describe what it wants up front, or just describe what happeend

use cli_rs::command::Command;

fn main() {
    Command::name("test")
        .arg("name", "Parth")
        .handler(|name| println!("test"));
}
