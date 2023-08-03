use cli_rs::{arg::Arg, command::Command, parser::Cmd};

fn main() {
    Command::name("cli-rs")
        .subcommand(
            Command::name("edit")
                .input(Arg::<String>::name("target"))
                .handler(|target| println!("editing target file: {}", target.get())),
        )
        .with_completions()
        .parse();
}
