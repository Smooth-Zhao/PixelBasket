pub mod basket;
pub mod db;
pub mod file;
pub mod util;
pub mod config;

use core::result::Result as CoreResult;
use std::error::Error;

pub type Result<T> = CoreResult<T, Box<dyn Error>>;
