# cli-rs

🚧 wip 🚧

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

# spec

```
command_path --flags positional_args --flags positional_args
```

things for now
+ args are all required and must be provided in order
+ flags are always optional (must impl default) and can be provided out of order
+ flags that aren't booleans follow the form `--key=value`
+ boolean that are boolean are parsed as either `--key`, `--key=false`, or `-k`
+ a command can have subcommands or args & flags, but not both

things for later:
+ support `--key value` 
+ can define an environment variable for flag values (cli specified value, env var fallback, then Default::default())
+ subcommands inherit any flags as their own flags
+ additionally all boolean flags can be grouped, such as `-rf`
+ list args (support for optional args)
+ detect invalid configuration at runtime
