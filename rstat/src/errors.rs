use std::fmt;
use std::io;
use std::num::TryFromIntError;

#[derive(Debug)]
pub struct StatsError {
    pub message: String,
}

impl StatsError {
    fn new(msg: &str) -> StatsError {
        StatsError{message: msg.to_string()}
    }
}

impl fmt::Display for StatsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<&str> for StatsError {
    fn from(msg: &str) -> Self {
        StatsError::new(msg)
    }
}

impl From<io::Error> for StatsError {
    fn from(error: io::Error) -> Self {
        StatsError::new(&error.to_string())
    }
}

impl From<TryFromIntError> for StatsError {
    fn from(error: TryFromIntError) -> Self {
        StatsError::new(&error.to_string())
    }
}
