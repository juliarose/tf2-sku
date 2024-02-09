//! Errors.

use std::fmt;
use std::num::{IntErrorKind, ParseIntError};

/// An error when parsing from a string.
#[derive(Debug)]
pub enum ParseError {
    /// An integer failed to parse.
    ParseInt {
        /// The key of the attribute.
        key: &'static str,
        /// The error from parsing the integer.
        error: ParseIntError,
    },
    /// The SKU format is not valid. Must begin with a defindex and a quality e.g. "5021;6".
    InvalidFormat,
    /// An attribute value is not valid.
    InvalidValue {
        /// The key of the attribute.
        key: &'static str,
        /// The value attempted to be converted.
        number: u32,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::ParseInt {
                key,
                error ,
            } => match *error.kind() {
                IntErrorKind::Empty => write!(f, "Value for {key} in SKU is empty."),
                IntErrorKind::InvalidDigit => write!(f, "Value for {key} in SKU contains invalid digit."),
                IntErrorKind::PosOverflow => write!(f, "Value for {key} in SKU overflows integer bounds."),
                IntErrorKind::NegOverflow => write!(f, "Value for {key} in SKU underflows integer bounds."),
                // shouldn't occur
                IntErrorKind::Zero => write!(f, "Value for {key} in SKU zero for non-zero type."),
                _ => write!(f, "Value for {key} in SKU could not be parsed: {error}"),
            },
            ParseError::InvalidFormat => write!(f, "Invalid SKU format. Must begin with a defindex followed by a quality e.g. \"5021;6\""),
            ParseError::InvalidValue {
                key,
                number,
            } => write!(f, "Unknown {key}: {number}"),
        }
    }
}