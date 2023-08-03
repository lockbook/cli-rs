use self::command0::Command0;
use crate::input::Input;
use crate::parser::ParseError;
use std::fmt::Write;
use std::str::FromStr;

pub mod command0;
pub mod command1;
pub mod command2;
pub mod command3;

pub type Command<'a> = Command0<'a>;

pub trait ParserInfo {
    fn docs(&self) -> &DocInfo;
    fn symbols(&mut self) -> Vec<&mut dyn Input>;
    fn subcommand_docs(&self) -> Vec<DocInfo>;
    fn parse_subcommand(&mut self, sub_idx: usize, tokens: &[String]) -> Result<(), ParseError>;
    fn complete_subcommand(&mut self, sub_idx: usize, tokens: &[String]) -> Result<(), ParseError>;
    fn call_handler(&mut self);
    fn push_parent(&mut self, parents: &[String]);
}

#[derive(Default, Debug, Clone)]
pub struct DocInfo {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) parents: Vec<String>,
}

impl DocInfo {
    pub fn cmd_path(&self) -> String {
        let mut path = String::new();
        for parent in &self.parents {
            write!(path, "{parent} ").unwrap();
        }

        write!(path, "{}", self.name).unwrap();
        path
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CompletionMode {
    Bash,
    Fish,
    Zsh,
}

impl FromStr for CompletionMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bash" => Ok(Self::Bash),
            "zsh" => Ok(Self::Zsh),
            "fish" => Ok(Self::Fish),
            _ => panic!("unsuppored shell"),
        }
    }
}

impl CompletionMode {
    // thanks @ad-tra
    pub fn print_completion(&self, name: &str) {
        let adapter = match self {
            CompletionMode::Bash => {
                format!(
                    r#"
_{name}_complete_()
{{
    _COMP_OUTPUTSTR=\"$( {name} complete -- \"${{COMP_WORDS[*]}}\" ${{COMP_CWORD}} )\"
    if test $? -ne 0; then
        return 1
    fi
    COMPREPLY=($( echo -n \"$_COMP_OUTPUTSTR\" ))
}}
complete -o nospace -F _{name}_complete_ {name} -E
                        "#
                )
            }
            CompletionMode::Fish => format!(
                r#"complete -c {name} -f --condition "not __fish_seen_subcommand_from file-command non-file-command" -a '({name} complete fish 0 (commandline -cp))'"#
            ),
            CompletionMode::Zsh => todo!(),
        };

        println!("{adapter}");
    }
}