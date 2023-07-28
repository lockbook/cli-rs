use std::str::FromStr;

use cli_rs::{
    arg::Arg,
    command::Command,
    parser::{Cmd, ParseError},
};

#[test]
fn basic_command() {
    let mut called = false;
    Command::name("test")
        .handler(|| called = true)
        .parse_args(&[])
        .unwrap();
    assert!(called);
}

#[test]
fn with_1_arg() {
    let mut input_name = String::default();

    Command::name("test")
        .input(Arg::str("name"))
        .handler(|name| input_name = name.get().clone())
        .parse_args(&["parth".to_string()])
        .unwrap();

    assert_eq!(input_name, "parth");
}

#[test]
fn with_2_args() {
    let mut name = String::default();
    let mut age = 0;

    Command::name("nameage")
        .input(Arg::str("name"))
        .input(Arg::i32("age"))
        .handler(|n, a| {
            name = n.get().clone();
            age = *a.get();
        })
        .parse_args(&["parth".to_string(), "27".to_string()])
        .unwrap();

    assert_eq!(name, "parth");
    assert_eq!(age, 27);
}

#[test]
fn missing_arg() {
    let mut name = String::default();
    let mut age = 0;

    let err: ParseError = Command::name("nameage")
        .input(Arg::str("name"))
        .input(Arg::i32("age"))
        .handler(|n, a| {
            name = n.get().clone();
            age = *a.get();
        })
        .parse_args(&["parth".to_string()])
        .unwrap_err();

    assert_eq!(err, ParseError::MissingArg);
    assert_eq!(name, String::default());
    assert_eq!(age, 0);
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FileType {
    Folder,
    Document,
}

impl FromStr for FileType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "folder" | "dir" => Ok(Self::Folder),
            "doc" | "document" => Ok(Self::Document),
            _ => unimplemented!(),
        }
    }
}

#[test]
fn with_custom_args() {
    let mut name = String::default();
    let mut ft = FileType::Folder;

    Command::name("create")
        .input(Arg::str("name"))
        .input(Arg::<FileType>::name("type"))
        .handler(|n, f| {
            name = n.get().clone();
            ft = *f.get();
        })
        .parse_args(&["todo.md".to_string(), "doc".to_string()])
        .unwrap();

    assert_eq!(ft, FileType::Document);
}
