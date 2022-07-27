use crate::Error;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;
