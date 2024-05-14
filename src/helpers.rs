//! Helper methods.

use crate::error::ParseError;
use tf2_enum::num_enum::TryFromPrimitive;

/// Parses an enum from a `&str` converted to a `u32`.
pub fn parse_enum_u32<T>(
    key: &'static str,
    s: &str,
) -> Result<T, ParseError>
where
    T: TryFromPrimitive<Primitive = u32>,
{
    let number = parse_u32(key, s)?;
    
    T::try_from_primitive(number)
        .map_err(|_| ParseError::InvalidValue {
            key,
            number,
        })
}

/// Parses a `&str` into a `u32`.
pub fn parse_u32(key: &'static str, value: &str) -> Result<u32, ParseError> {
    value.parse::<u32>()
        .map_err(|error| ParseError::ParseInt {
            key,
            error,
        })
}