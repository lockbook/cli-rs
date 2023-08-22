use std::fmt::Display;

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug, PartialEq)]
pub struct CliError {
    pub msg: String,
    pub status: i32,
}

impl<D: Display> From<D> for CliError {
    fn from(value: D) -> Self {
        let msg = value.to_string();
        Self { msg, status: 1 }
    }
}

pub trait Exit {
    type O;

    fn exit(self);
    fn exit_if_err(self) -> Self::O;
    fn exit_silently(self);
}

impl<O> Exit for CliResult<O> {
    type O = O;

    fn exit(self) {
        match self {
            Ok(_) => std::process::exit(0),
            Err(err) => {
                eprintln!("{}", err.msg);
                std::process::exit(err.status);
            }
        }
    }

    fn exit_if_err(self) -> Self::O {
        match self {
            Ok(o) => o,
            Err(err) => {
                eprintln!("{}", err.msg);
                std::process::exit(err.status);
            }
        }
    }

    fn exit_silently(self) {
        let status = match self {
            Ok(_) => 0,
            Err(e) => e.status,
        };

        std::process::exit(status);
    }
}
