use crate::{arg::Arg, flag::Flag};

pub trait Input {}

impl<T> Input for Arg<T> {}

impl<T> Input for Flag<T> {}
