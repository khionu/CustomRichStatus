use std::{error::Error, fmt, fmt::Display};

#[derive(Debug)]
pub struct GnrError {
    pub msg: &'static str,
    pub handling: Handling,
    pub cause: Option<Box<dyn Error>>,
}

#[derive(Debug)]
pub enum Handling {
    Print,
    Exit,
    Crash,
}

impl Display for GnrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl GnrError {
    pub fn new(msg: &'static str, handling: Handling) -> Box<GnrError> {
        // Note: Using String here is a hack -- it doesn't matter which is called
        Self::internal_new::<String>(msg, handling, None)
    }

    pub fn new_with_cause<T: Into<Box<dyn Error>>>(msg: &'static str, handling: Handling, cause: T) -> Box<GnrError> {
        Self::internal_new(msg, handling, Some(cause))
    }

    fn internal_new<T: Into<Box<dyn Error>>>(msg: &'static str, handling: Handling, cause: Option<T>) -> Box<GnrError> {
        Box::new(GnrError {
            msg,
            handling,
            cause: cause.map(|err| err.into()),
        })
    }
}

impl Error for GnrError {
    fn cause<'a>(&self) -> Option<&Error> {
        match &self.cause {
            None => None,
            &Some(ref v) => Some(v.as_ref())
        }
    }
}