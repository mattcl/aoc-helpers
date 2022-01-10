use std::num::ParseIntError;

/// Convenience type of `Result<T, AocError>`
pub type Result<T> = std::result::Result<T, AocError>;

/// AocError enumerates all possible errors returned by this library
#[derive(Debug)]
pub enum AocError {
    /// Used to indicate a `Grid` could not be constructed
    GridConstructionError(String),

    /// Used to indicate a specified input did not exist
    InputMissing(String),

    /// Used to indicate a `Direction` could not be parsed
    ParseDirectionError(String),

    /// Used to indicate a `Location` could not be parsed
    ParseLocationError(String),

    // Generic Errors below
    /// Represents all other cases of IO errors
    IOError(std::io::Error),

    /// Represents all other cases of OsStringErrs
    OsStringErr(std::ffi::OsString),

    /// Represents all other cases of ParseIntErrors
    ParseIntError(ParseIntError),

    /// Represents all other cases VarErrors
    VarError(std::env::VarError),
}

impl std::error::Error for AocError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            AocError::GridConstructionError(_) => None,
            AocError::InputMissing(_) => None,
            AocError::ParseDirectionError(_) => None,
            AocError::ParseLocationError(_) => None,

            // Generic
            AocError::IOError(ref err) => Some(err),
            AocError::OsStringErr(_) => None,
            AocError::ParseIntError(ref err) => Some(err),
            AocError::VarError(ref err) => Some(err),
        }
    }
}

impl std::fmt::Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            AocError::GridConstructionError(ref msg) => {
                write!(f, "unable to construct Grid: {}", msg)
            }
            AocError::InputMissing(ref input) => {
                write!(f, "specified input was missing: {}", input)
            }
            AocError::ParseDirectionError(ref msg) => {
                write!(f, "failed to parse Direction: {}", msg)
            }
            AocError::ParseLocationError(ref msg) => write!(f, "failed to parse Location: {}", msg),

            // Generic
            AocError::IOError(ref err) => err.fmt(f),
            AocError::OsStringErr(ref err) => write!(f, "OsString error: {:?}", err),
            AocError::ParseIntError(ref err) => err.fmt(f),
            AocError::VarError(ref err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for AocError {
    fn from(err: std::io::Error) -> AocError {
        AocError::IOError(err)
    }
}

impl From<ParseIntError> for AocError {
    fn from(err: ParseIntError) -> AocError {
        AocError::ParseIntError(err)
    }
}

impl From<std::ffi::OsString> for AocError {
    fn from(err: std::ffi::OsString) -> AocError {
        AocError::OsStringErr(err)
    }
}

impl From<std::env::VarError> for AocError {
    fn from(err: std::env::VarError) -> AocError {
        AocError::VarError(err)
    }
}
