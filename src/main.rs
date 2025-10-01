use cli_rs::{arg::Arg, cli_error::Exit, command::Command, flag::Flag, parser::Cmd};

fn main() {
    let files = vec![
        "todo.md".to_string(),
        "test.md".to_string(),
        "a.md".to_string(),
        "abba.md".to_string(),
    ];

    //eprintln!("{:#?}", std::env::args().collect::<Vec<String>>());

    Command::name("cli-rs")
        .subcommand(
            Command::name("edit")
                .input(Flag::<String>::new("editor").completor(|prompt| {
                    Ok(["vim", "nvim", "nano", "sublime", "code"]
                        .iter()
                        .filter(|editor| editor.starts_with(prompt))
                        .map(|s| s.to_string())
                        .collect())
                }))
                .input(Flag::<String>::new("force"))
                .input(Arg::<String>::name("target").completor(|prompt| {
                    Ok(files
                        .clone()
                        .into_iter()
                        .filter(|file| file.starts_with(prompt))
                        .collect())
                }))
                .handler(|_, _, target| {
                    println!("editing target file: {}", target.get());
                    Ok(())
                }),
        )
        .with_completions()
        .parse()
        .exit();
}

// todo: cli-rs edit<tab> crashes
// todo: with the force flag completions stop working
