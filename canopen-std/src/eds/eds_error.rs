use std::{fmt, io};

/// An error encountered while parsing an EDS/DCF file.
#[derive(Debug)]
pub enum EdsError {
    /// The file could not be read.
    Io(io::Error),
    /// An object section was missing a required field.
    MissingField {
        /// The `[section]` the field was expected in.
        section: String,
        /// The absent field name.
        field: &'static str,
    },
    /// An `AccessType` value was not one of `ro`/`wo`/`rw`/`rwr`/`rww`/`const`.
    UnknownAccessType(String),
    /// A `DefaultValue`/`ParameterValue` could not be parsed for its type.
    InvalidValue {
        /// The `[section]` the value came from.
        section: String,
        /// The offending text.
        value: String,
    },
    UnknownObjectType(String),
}

impl fmt::Display for EdsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EdsError::Io(e) => write!(f, "reading EDS file: {e}"),
            EdsError::MissingField { section, field } => {
                write!(f, "section [{section}] is missing required field `{field}`")
            }
            EdsError::UnknownAccessType(s) => write!(f, "unknown AccessType `{s}`"),
            EdsError::InvalidValue { section, value } => {
                write!(f, "section [{section}] has an invalid value `{value}`")
            }
            EdsError::UnknownObjectType(s) => write!(f, "unknown ObjectType `{s}`"),
        }
    }
}

impl std::error::Error for EdsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EdsError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for EdsError {
    fn from(e: io::Error) -> Self {
        EdsError::Io(e)
    }
}
