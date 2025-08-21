//! Helper methods.

use crate::error::ParseError;
use tf2_enum::TryFromPrimitive;
use tf2_enum::Spell;

/// Parses an enum from a `&str` converted to a `u32`.
#[inline]
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
#[inline]
pub fn parse_u32(
    key: &'static str,
    value: &str,
) -> Result<u32, ParseError> {
    value.parse::<u32>()
        .map_err(|error| ParseError::ParseInt {
            key,
            error,
        })
}

#[inline]
pub fn spell_label(spell: &Spell) -> &'static str {
    match spell.attribute_defindex() {
        Spell::DEFINDEX_VOICES_FROM_BELOW => "voices",
        Spell::DEFINDEX_EXORCISM => "exorcism",
        Spell::DEFINDEX_PUMPKIN_BOMBS => "pumpkinbombs",
        Spell::DEFINDEX_HALLOWEEN_FIRE => "halloweenfire",
        Spell::DEFINDEX_PAINT => "paintspell",
        Spell::DEFINDEX_FOOTPRINTS => "footprints",
        _ => "",
    }
}
