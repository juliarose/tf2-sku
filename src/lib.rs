//! # tf2-sku
//! 
//! SKU parser for Team Fortress 2 items.
//! 
//! ## Usage
//! ```
//! use tf2_sku::SKU;
//! use tf2_sku::tf2_enum::{Quality, KillstreakTier, Spell};
//! 
//! let sku = "264;11;kt-1".parse::<SKU>().unwrap();
//! 
//! assert_eq!(sku.defindex, 264);
//! assert_eq!(sku.quality, Quality::Strange);
//! assert_eq!(sku.killstreak_tier, Some(KillstreakTier::Killstreak));
//! assert_eq!(sku.to_string(), "264;11;kt-1");
//! 
//! // Also supports spells and strange parts
//! let sku = "627;6;footprints-2".parse::<SKU>().unwrap();
//! 
//! assert!(sku.spells.contains(&Spell::HeadlessHorseshoes));
//! ```

#![warn(missing_docs)]

pub mod error;

mod helpers;
mod spell_set;
mod strange_part_set;

pub use spell_set::SpellSet;
pub use strange_part_set::StrangePartSet;
pub use tf2_enum;

use error::ParseError;
use helpers::{parse_enum_u32, parse_u32};

use std::fmt;
use std::convert::TryFrom;
use std::hash::Hash;
use std::str::FromStr;
use tf2_enum::{Quality, KillstreakTier, Wear, Paint, Sheen, Killstreaker, Spell, FootprintsSpell, PaintSpell};
use serde::{Serialize, Serializer};
use serde::de::{self, Visitor};

/// Trait for converting to a SKU string.
pub trait SKUString {
    /// Converts to a SKU string.
    fn to_sku_string(&self) -> String;
}

/// A SKU containing detailed fields to identify an item.
/// 
/// # Examples
/// ```
/// use tf2_sku::SKU;
/// use tf2_enum::{KillstreakTier, Sheen, Killstreaker, Quality};
/// 
/// let sku = SKU {
///     defindex: 264,
///     quality: Quality::Strange,
///     killstreak_tier: Some(KillstreakTier::Professional),
///     sheen: Some(Sheen::TeamShine),
///     killstreaker: Some(Killstreaker::FireHorns),
///     ..Default::default()
/// };
/// 
/// assert_eq!(sku.to_string(), "264;11;kt-3;ks-1;ke-2002");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SKU {
    /// This can be negative at times to refer to items that are not defined in the schema e.g. 
    /// ["Random Craft Hat"](https://marketplace.tf/items/tf2/-100;6).
    pub defindex: i32,
    /// The quality of the item.
    pub quality: Quality,
    /// Whether the item is craftable.
    pub craftable: bool,
    /// Whether the item is australium.
    pub australium: bool,
    /// Whether the item is strange. Not to be confused with strange quality items.
    pub strange: bool,
    /// Whether the item is festivized.
    pub festivized: bool,
    /// The particle effect value of the item.
    pub particle: Option<u32>,
    /// The skin value of the item.
    pub skin: Option<u32>,
    /// The killstreak tier of the item.
    pub killstreak_tier: Option<KillstreakTier>,
    /// The wear of the item.
    pub wear: Option<Wear>,
    /// The target defindex of the item.
    pub target_defindex: Option<u32>,
    /// The output defindex of the item.
    pub output_defindex: Option<u32>,
    /// The output quality of the item.
    pub output_quality: Option<Quality>,
    /// The craft number of the item.
    pub craft_number: Option<u32>,
    /// The crate number of the item.
    pub crate_number: Option<u32>,
    /// The paint of the item.
    pub paint: Option<Paint>,
    /// The sheen of the item.
    pub sheen: Option<Sheen>,
    /// The killstreaker of the item.
    pub killstreaker: Option<Killstreaker>,
    /// The spells of the item.
    pub spells: SpellSet,
    /// The strange parts of the item.
    pub strange_parts: StrangePartSet,
}

/// Creates a SKU with default values. All `Option` fields will be `None`, and all `bool` fields 
/// will be `false`, with the exception of craftable, which is `true`. `quality` will be 
/// [`Quality::Normal`]. 
impl Default for SKU {
    fn default() -> Self {
        Self {
            defindex: 0,
            quality: Quality::Normal,
            craftable: true,
            australium: false,
            strange: false,
            festivized: false,
            particle: None,
            skin: None,
            killstreak_tier: None,
            wear: None,
            target_defindex: None,
            output_defindex: None,
            output_quality: None,
            craft_number: None,
            crate_number: None,
            paint: None,
            sheen: None,
            killstreaker: None,
            spells: SpellSet::default(),
            strange_parts: StrangePartSet::default(),
        }
    }
}

impl SKU {
    /// Creates a new SKU using the given `defindex` and `quality`. All `Option` fields will be 
    /// `None`, and all `bool` fields will be `false`, with the exception of craftable, which is 
    /// `true`. 
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::SKU;
    /// use tf2_enum::Quality;
    /// 
    /// let sku = SKU::new(264, Quality::Strange);
    /// assert_eq!(sku.to_string(), "264;11");
    /// ```
    pub fn new(
        defindex: i32,
        quality: Quality,
    ) -> Self {
        Self {
            defindex,
            quality,
            ..Self::default()
        }
    }
    
    /// Parses attributes from a string, ignoring failures; always produces an output regardless 
    /// of input. It's advised to use [`SKU::from_str`] over this method to ensure predictable 
    /// output. If no `defindex` is detected, it will default to `-1`. `quality` defaults to 
    /// [`Quality::Rarity2`]. If the SKU is properly formatted this produces identical output as 
    /// [`SKU::from_str`].
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::SKU;
    /// use tf2_enum::Quality;
    /// 
    /// let sku = SKU::parse_attributes("12;u43;kt-0;gibus");
    /// assert_eq!(sku.defindex, 12);
    /// assert_eq!(sku.quality, Quality::Rarity2);
    /// assert_eq!(sku.particle, Some(43));
    /// assert!(sku.killstreak_tier.is_none());
    /// 
    /// // Valid SKU.
    /// let sku = "200;11;australium;kt-3".parse::<SKU>().unwrap();
    /// // Produces the same output if the SKU is valid.
    /// assert_eq!(SKU::parse_attributes("200;11;australium;kt-3"), sku);
    /// // Invalid quality, produces a different output.
    /// assert_ne!(SKU::parse_attributes("200;453;australium;kt-3"), sku);
    /// ```
    pub fn parse_attributes(string: &str) -> Self {
        let mut parsed = Self::default();
        let mut sku_split = string.split(';');
        let defindex_str = sku_split.next()
            .unwrap_or_default();
        let quality_str = sku_split.next()
            .unwrap_or_default();
        
        if let Ok(defindex) = defindex_str.parse::<i32>() {
            parsed.defindex = defindex;
        } else {
            parsed.defindex = -1;
            parse_sku_element(&mut parsed, defindex_str).ok();
        }
        
        if let Ok(quality) = parse_enum_u32::<Quality>("quality", quality_str) {
            parsed.quality = quality;
        } else {
            parsed.quality = Quality::Rarity2;
            parse_sku_element(&mut parsed, quality_str).ok();
        }
        
        for element in sku_split {
            parse_sku_element(&mut parsed, element).ok();
        }
        
        parsed
    }
}

/// This is the same as `to_string`.
impl SKUString for SKU {
    fn to_sku_string(&self) -> String {
        self.to_string()
    }
}

/// This is the same as `to_string`.
impl SKUString for &SKU {
    fn to_sku_string(&self) -> String {
        (*self).to_sku_string()
    }
}

/// Formats SKU attributes into a string.
/// 
/// # Examples
/// ```
/// use tf2_sku::SKU;
/// use tf2_enum::{Quality, KillstreakTier};
/// 
/// let mut sku = SKU::new(264, Quality::Strange);
/// 
/// sku.killstreak_tier = Some(KillstreakTier::Professional);
/// 
/// assert_eq!(sku.to_string(), "264;11;kt-3");
/// ```
impl fmt::Display for SKU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{};{}", self.defindex, u32::from(self.quality))?;
        
        if let Some(particle) = self.particle {
            write!(f, ";u{}", particle)?;
        }
        
        if !self.craftable {
            write!(f, ";uncraftable")?;
        }
        
        if self.australium {
            write!(f, ";australium")?;
        }
        
        if self.strange {
            write!(f, ";strange")?;
        }
        
        if let Some(wear) = self.wear {
            write!(f, ";w{}", u32::from(wear))?;
        }
        
        if let Some(skin) = self.skin {
            write!(f, ";pk{}", skin)?;
        }
        
        if let Some(killstreak_tier) = self.killstreak_tier {
            write!(f, ";kt-{}", u32::from(killstreak_tier))?;
        }
        
        if self.festivized {
            write!(f, ";festive")?;
        }
        
        if let Some(crate_number) = self.crate_number {
            write!(f, ";c{}", crate_number)?;
        }
        
        if let Some(craft_number) = self.craft_number {
            write!(f, ";n{}", craft_number)?;
        }
        
        if let Some(target_defindex) = self.target_defindex {
            write!(f, ";td-{}", target_defindex)?;
        }
        
        if let Some(output_defindex) = self.output_defindex {
            write!(f, ";od-{}", output_defindex)?;
        }
        
        if let Some(output_quality) = self.output_quality {
            write!(f, ";oq-{}", u32::from(output_quality))?;
        }
        
        if let Some(paint) = self.paint {
            write!(f, ";p{}", u32::from(paint))?;
        }
        
        if let Some(sheen) = self.sheen {
            write!(f, ";ks-{}", u32::from(sheen))?;
        }
        
        if let Some(killstreaker) = self.killstreaker {
            write!(f, ";ke-{}", u32::from(killstreaker))?;
        }
        
        for strange_part in self.strange_parts {
            write!(f, ";sp-{}", u32::from(strange_part))?;
        }
        
        for spell in self.spells {
            write!(f, ";{}", spell_set::spell_label(&spell))?;
            
            if let Some(value) = spell.attribute_value() {
                write!(f, "-{}", value)?;
            }
        }
        
        Ok(())
    }
}

/// Attempts to parse a SKU from a string. Fails if SKU contains invalid attribute e.g. a 
/// [`Quality`] not defined, `"kt-5"` is an invalid [`KillstreakTier`]. Ignores unknown 
/// attributes.
/// 
/// # Examples
/// ```
/// use tf2_sku::SKU;
/// use tf2_enum::{Quality, KillstreakTier};
/// 
/// let sku = "264;11;kt-3".parse::<SKU>().unwrap();
/// 
/// assert_eq!(sku.defindex, 264);
/// assert_eq!(sku.quality, Quality::Strange);
/// assert_eq!(sku.killstreak_tier, Some(KillstreakTier::Professional));
/// ```
impl TryFrom<&str> for SKU {
    type Error = ParseError;
        
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut sku_split = s.split(';');
        let defindex_str = sku_split.next()
            .ok_or(ParseError::InvalidFormat)?;
        let quality_str = sku_split.next()
            .ok_or(ParseError::InvalidFormat)?;
        let defindex = defindex_str.parse()
            .map_err(|error| ParseError::ParseInt {
                key: "defindex",
                error,
            })?;
        let quality = parse_enum_u32("quality", quality_str)?;
        let mut parsed = SKU::new(defindex, quality);
        
        for element in sku_split {
            parse_sku_element(&mut parsed, element)?;
        }
        
        Ok(parsed)
    }
}

impl TryFrom<String> for SKU {
    type Error = ParseError;
    
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl TryFrom<&String> for SKU {
    type Error = ParseError;
    
    fn try_from(s: &String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl FromStr for SKU {
    type Err = ParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

/// Parses a single SKU attribute.
fn parse_sku_element(
    parsed: &mut SKU,
    element: &str,
) -> Result<(), ParseError> {
    // This is the byte length of the string. NOT the character length.
    let mut split_at = element.len();
    
    // Walk back through chars until a non-digit is found
    for c in element.chars().rev() {
        if c.is_ascii_digit() {
            split_at -= 1;
        } else {
            break;
        }
    }
    
    // Split at where the numeric value begins (`value` will be an empty string if no digit was found)
    // This shouldn't cause issues with strings that contain varying byte lengths. If the last 
    // character is multi-byte it is not a valid ascii digit, so it will stop immediately and `split_at`
    // will be the total byte length of the string.
    let (name, value) = element.split_at(split_at);
    
    match name {
        "u" => parsed.particle = Some(parse_u32("particle", value)?),
        "w" => parsed.wear = Some(parse_enum_u32("wear", value)?),
        "n" => parsed.craft_number = Some(parse_u32("craft number", value)?),
        "c" => parsed.crate_number = Some(parse_u32("crate number", value)?),
        "p" => parsed.paint = Some(parse_enum_u32("paint", value)?),
        "pk" => parsed.skin = Some(parse_u32("skin", value)?),
        "kt-" => parsed.killstreak_tier = Some(parse_enum_u32("killstreak tier", value)?),
        "td-" => parsed.target_defindex = Some(parse_u32("target defindex", value)?),
        "od-" => parsed.output_defindex = Some(parse_u32("output defindex", value)?),
        "oq-" => parsed.output_quality = Some(parse_enum_u32("output quality", value)?),
        "ks-" => parsed.sheen = Some(parse_enum_u32("sheen", value)?),
        "ke-" => parsed.killstreaker = Some(parse_enum_u32("killstreaker", value)?),
        "sp-" => {
            parsed.strange_parts.insert(parse_enum_u32("strange part", value)?).ok();
        },
        "footprints-" => {
            let spell = parse_enum_u32::<FootprintsSpell>("footprints spell", value)?;
            
            parsed.spells.insert(spell.into()).ok();
        },
        "paintspell-" => {
            let spell = parse_enum_u32::<PaintSpell>("paint spell", value)?;
            
            parsed.spells.insert(spell.into()).ok();
        },
        "voices" => {
            parsed.spells.insert(Spell::VoicesFromBelow).ok();
        },
        "exorcism" => {
            parsed.spells.insert(Spell::Exorcism).ok();
        },
        "halloweenfire" => {
            parsed.spells.insert(Spell::HalloweenFire).ok();
        },
        "pumpkinbombs" => {
            parsed.spells.insert(Spell::PumpkinBombs).ok();
        },
        "uncraftable" => parsed.craftable = false,
        "australium" => parsed.australium = true,
        "strange" => parsed.strange = true,
        "festive" => parsed.festivized = true,
        // ignore
        _ => {},
    }
    
    Ok(())
}

impl Serialize for SKU {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> de::Deserialize<'de> for SKU {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct SKUVisitor;

        impl<'de> Visitor<'de> for SKUVisitor {
            type Value = SKU;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a string")
            }
            
            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Self::Value::try_from(s).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(SKUVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json::{self, json};
    use std::sync::Arc;
    use tf2_enum::StrangePart;
    
    #[derive(Serialize, Deserialize)]
    struct Item {
        sku: SKU,
    }
    
    #[test]
    fn golden_frying_pan_correct() {
        assert_eq!("1071;11;kt-3".parse::<SKU>().unwrap(), SKU {
            defindex: 1071,
            quality: Quality::Strange,
            killstreak_tier: Some(KillstreakTier::Professional),
            ..SKU::default()
        });
    }
    
    #[test]
    fn professional_unusual_killstreak_skin() {
        assert_eq!("424;15;u703;w3;pk307;kt-3;ks-1;ke-2008".parse::<SKU>().unwrap(), SKU {
            defindex: 424,
            quality: Quality::DecoratedWeapon,
            particle: Some(703),
            skin: Some(307),
            killstreak_tier: Some(KillstreakTier::Professional),
            wear: Some(Wear::FieldTested),
            sheen: Some(Sheen::TeamShine),
            killstreaker: Some(Killstreaker::HypnoBeam),
            ..SKU::default()
        });
    }
    
    #[test]
    fn attribute_with_four_byte_utf8_char_is_ignored() {
        assert!("1071;1;u-🍌🍌122;🍌🍌".parse::<SKU>().unwrap().particle.is_none());
        assert!("1071;1;u🍌122;🍌🍌".parse::<SKU>().unwrap().particle.is_none());
        assert!("1071;1;u🍌122🍌;🍌🍌".parse::<SKU>().unwrap().particle.is_none());
    }
    #[test]
    
    fn parses_attributes() {
        let sku = SKU::parse_attributes("u43;;;pk1;kt-0;gibus🍌");
        
        assert_eq!(sku.defindex, -1);
        assert_eq!(sku.quality, Quality::Rarity2);
        assert_eq!(sku.particle, Some(43));
        assert_eq!(sku.skin, Some(1));
        assert!(sku.killstreak_tier.is_none());
    }
    
    #[test]
    fn bad_quality_is_err() {
        assert!("1071;122".parse::<SKU>().is_err());
    }
    
    #[test]
    fn empty_quality_is_err() {
        assert!("1;5;u;pk1".parse::<SKU>().is_err());
    }
    
    #[test]
    fn unknown_attribute_is_ok() {
        assert!("1;5;superspecial".parse::<SKU>().is_ok());
        assert_eq!("1;5;superspecial".parse::<SKU>().unwrap().to_string(), "1;5");
    }
    
    #[test]
    fn bad_quality_is_err_check_error_key() {
        if let ParseError::InvalidValue { key, number } = "1071;122".parse::<SKU>().unwrap_err() {
            assert_eq!(key, "quality");
            assert_eq!(number, 122);
        } else {
            panic!("wrong error");
        }
    }
    
    #[test]
    fn negative_defindex_is_ok() {
        assert!("-1;11".parse::<SKU>().is_ok());
    }
    
    #[test]
    fn paint_kit_correct() {
        assert!("16310;15;u703;w2;pk310".parse::<SKU>().is_ok());
    }

    #[test]
    fn deserializes_from_json() {
        let item = serde_json::from_value::<Item>(json!({
            "sku": "16310;15;u703;w2;pk310"
        })).unwrap();

        assert_eq!(item.sku.defindex, 16310);
    }
    
    #[test]
    fn deserializes_to_json() {
        let sku = "16310;15;u703;w2;pk310".parse::<SKU>().unwrap();
        let s = serde_json::to_string(&Item { sku }).unwrap();

        assert_eq!(s, r#"{"sku":"16310;15;u703;w2;pk310"}"#);
    }
    
    #[test]
    fn to_sku_string_in_arc() {
        let sku = Arc::new("16310;15;u703;w2;pk310".parse::<SKU>().unwrap());
        
        assert_eq!(sku.as_ref().to_sku_string(), "16310;15;u703;w2;pk310");
    }
    
    #[test]
    fn parses_spells_sku() {
        let sku = "627;6;footprints-2;voices".parse::<SKU>().unwrap();
        
        assert_eq!(sku.spells, SpellSet::from([
            Some(Spell::HeadlessHorseshoes),
            Some(Spell::VoicesFromBelow),
        ]));
    }
    
    #[test]
    fn parses_strange_parts() {
        let sku = "627;6;sp-36;sp-37".parse::<SKU>().unwrap();
        
        assert_eq!(sku.strange_parts, StrangePartSet::from([
            Some(StrangePart::SappersRemoved),
            Some(StrangePart::CloakedSpiesKilled),
            None,
        ]));
    }
}
