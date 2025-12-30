use std::fmt;

use crate::domain::causes::Cause;


#[derive(Debug, Clone)]
pub struct SimpleCause(String);

impl From<Cause> for SimpleCause {
    fn from(value: Cause) -> Self {
        Self(value.name)
    }
}

impl fmt::Display for SimpleCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}