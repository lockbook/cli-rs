# cli-rs

A library to help you quickly write expressive CLIs. Built from the ground up with advanced features like [dynamic completions](https://github.com/clap-rs/clap/issues/1232) in mind.

```rust
Command::name("lockbook")
   .subcommand(
       Command::name("edit")
           .input(Arg::new("target"))
           .handler(|target: Arg<String>| println!("editing target file: {}", target.get())),
   )
   .parse();
```

Specify complicated arguments that are used often:

```rust
let docs = Arg::<Uuid>::new("target-file")
    .description("A uuid or path of a lockbook document")
    .parser(|str| {...})
    .completions(|current_str, cursor_loc| {...});
```

cli-rs will automatically generate contextual help messages, and man pages.
cli-rs will also generate a tiny completions file for every shell which will call your CLI, moving as much of the completion logic into Rust as possible.
