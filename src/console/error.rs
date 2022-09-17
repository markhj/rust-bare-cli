use std::fmt;

impl fmt::Display for ConsoleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str : &str = match *self {
            ConsoleError::NoInput => "NoInput",
            ConsoleError::UnknownInput => "UnknownInput",
            ConsoleError::InvalidSyntax => "InvalidSyntax",
            //_ => "Unknown",
        };

        return write!(f, "{}", str);
    }
}

pub enum ConsoleError {
    NoInput,
    UnknownInput,
    InvalidSyntax,
}
