use cli_rs::{arg::Arg, command::Command, flag::Flag, parser::Cmd};

fn main() {
    let files = vec![
        "todo.md".to_string(),
        "test.md".to_string(),
        "a.md".to_string(),
        "abba.md".to_string(),
    ];

    Command::name("cli-rs")
        .subcommand(
            Command::name("edit")
                .input(Flag::<String>::new("edit"))
                .input(Arg::<String>::name("target").completor(|prompt| {
                    files
                        .clone()
                        .into_iter()
                        .filter(|file| file.starts_with(prompt))
                        .collect()
                }))
                .handler(|edit, target| println!("editing target file: {}", target.get())),
        )
        .with_completions()
        .parse();
}
