use core::fmt;

#[derive(Debug, Clone)]
pub enum DisplayError {
    CouldNotCreate,
}

impl fmt::Display for DisplayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not create display")
    }
}
