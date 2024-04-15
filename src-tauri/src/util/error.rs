use core::result::Result as CoreResult;
use std::error::Error;
use std::fmt::{Debug, Display};

use crate::error;

pub type Result<T> = CoreResult<T, Box<dyn Error>>;

pub trait ErrorHandle<T> {
    fn print_error(self) -> Option<T>;
}

impl<T, E: Debug + Display> ErrorHandle<T> for CoreResult<T, E> {
    fn print_error(self) -> Option<T> {
        match self {
            Ok(t) => Some(t),
            Err(e) => {
                error!("{e}");
                None
            }
        }
    }
}
