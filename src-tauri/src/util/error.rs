use std::backtrace::Backtrace;
use std::fmt::{Debug, Display};

use crate::error;

pub trait ErrorHandle<T> {
    fn print_error(self) -> Option<T>;
}

impl<T, E: Debug + Display> ErrorHandle<T> for Result<T, E> {
    fn print_error(self) -> Option<T> {
        match self {
            Ok(t) => Some(t),
            Err(e) => {
                error!("{e}\ntrace:\n{}", Backtrace::capture());
                None
            }
        }
    }
}
