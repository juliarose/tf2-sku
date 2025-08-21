//! SKU.

use crate::error::ParseError;
use crate::helpers::{parse_enum_u32, parse_u32, spell_label};
use std::convert::TryFrom;
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;
use serde::{Serialize, Serializer};
use serde::de::{self, Visitor};
use tf2_enum::{
    AttributeSet,
    FootprintsSpell,
    KillstreakTier,
    Killstreaker,
    Paint,
    PaintSpell,
    Quality,
    Sheen,
    Spell,
    SpellSet,
    StrangePartSet,
    Wear,
};

const KEY_QUALITY: &str = "quality";
const KEY_DEFINDEX: &str = "defindex";
const KEY_PARTICLE: &str = "particle";
const KEY_WEAR: &str = "wear";
const KEY_CRAFT_NUMBER: &str = "craft number";
const KEY_CRATE_NUMBER: &str = "crate number";
const KEY_PAINT: &str = "paint";
const KEY_SKIN: &str = "skin";
const KEY_KILLSTREAK_TIER: &str = "killstreak tier";
const KEY_TARGET_DEFINDEX: &str = "target defindex";
const KEY_OUTPUT_DEFINDEX: &str = "output defindex";
const KEY_OUTPUT_QUALITY: &str = "output quality";
const KEY_SHEEN: &str = "sheen";
const KEY_KILLSTREAKER: &str = "killstreaker";
const KEY_STRANGE_PART: &str = "strange part";
const KEY_FOOTPRINTS_SPELL: &str = "footprints spell";
const KEY_PAINT_SPELL: &str = "paint spell";

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
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

impl Default for SKU {
    /// Creates a SKU with default values. All `Option` fields will be `None`, and all `bool` fields
    /// will be `false`, with the exception of craftable, which is `true`. `quality` will be
    /// [`Quality::Normal`]. 
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
    /// [`Quality::Normal`]. If the SKU is properly formatted this produces identical output as
    /// [`SKU::from_str`].
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::SKU;
    /// use tf2_enum::Quality;
    /// 
    /// let sku = SKU::parse_attributes("12;u43;kt-0;gibus");
    /// assert_eq!(sku.defindex, 12);
    /// assert_eq!(sku.quality, Quality::Normal);
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
            
            if let Ok(quality) = parse_enum_u32::<Quality>(KEY_QUALITY, quality_str) {
                parsed.quality = quality;
            } else {
                parsed.quality = Quality::Normal;
                parse_sku_element(&mut parsed, quality_str).ok();
            }
        } else {
            parsed.defindex = -1;
            parsed.quality = Quality::Normal;
            parse_sku_element(&mut parsed, defindex_str).ok();
            parse_sku_element(&mut parsed, quality_str).ok();
        }
        
        for element in sku_split {
            parse_sku_element(&mut parsed, element).ok();
        }
        
        parsed
    }
}

impl SKUString for SKU {
    /// This is the same as `to_string`.
    fn to_sku_string(&self) -> String {
        self.to_string()
    }
}

impl SKUString for &SKU {
    /// This is the same as `to_string`.
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
        write!(f, "{};{}", self.defindex, self.quality as u32)?;
        
        if let Some(particle) = self.particle {
            write!(f, ";u{particle}")?;
        }
        
        if !self.craftable {
            f.write_str(";uncraftable")?;
        }
        
        if self.australium {
            f.write_str(";australium")?;
        }
        
        if self.strange {
            f.write_str(";strange")?;
        }
        
        if let Some(wear) = self.wear {
            write!(f, ";w{}", wear as u32)?;
        }
        
        if let Some(skin) = self.skin {
            write!(f, ";pk{skin}")?;
        }
        
        if let Some(killstreak_tier) = self.killstreak_tier {
            write!(f, ";kt-{}", killstreak_tier as u32)?;
        }
        
        if self.festivized {
            write!(f, ";festive")?;
        }
        
        if let Some(crate_number) = self.crate_number {
            write!(f, ";c{crate_number}")?;
        }
        
        if let Some(craft_number) = self.craft_number {
            write!(f, ";n{craft_number}")?;
        }
        
        if let Some(target_defindex) = self.target_defindex {
            write!(f, ";td-{target_defindex}")?;
        }
        
        if let Some(output_defindex) = self.output_defindex {
            write!(f, ";od-{output_defindex}")?;
        }
        
        if let Some(output_quality) = self.output_quality {
            write!(f, ";oq-{}", output_quality as u32)?;
        }
        
        if let Some(paint) = self.paint {
            write!(f, ";p{}", paint as u32)?;
        }
        
        if let Some(sheen) = self.sheen {
            write!(f, ";ks-{}", sheen as u32)?;
        }
        
        if let Some(killstreaker) = self.killstreaker {
            write!(f, ";ke-{}", killstreaker as u32)?;
        }
        
        for strange_part in self.strange_parts {
            write!(f, ";sp-{}", strange_part as u32)?;
        }
        
        for spell in self.spells {
            if let Some(value) = spell.attribute_id() {
                write!(f, ";{}-{}", spell_label(&spell), value)?;
            } else {
                write!(f, ";{}", spell_label(&spell))?;
            }
        }
        
        Ok(())
    }
}

impl TryFrom<&str> for SKU {
    type Error = ParseError;
    
    /// Attempts to parse a SKU from a string.
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
    /// 
    /// # Errors
    /// - The SKU does not contain a defindex and quality.
    /// - The SKU contains an invalid attribute e.g. a  [`Quality`] not defined.
    /// - An expected integer failed to parse.
    /// 
    /// Unknown attributes are ignored.
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut sku_split = s.split(';');
        let defindex_str = sku_split.next()
            .ok_or(ParseError::InvalidFormat)?;
        let quality_str = sku_split.next()
            .ok_or(ParseError::InvalidFormat)?;
        let defindex = defindex_str.parse()
            .map_err(|error| ParseError::ParseInt {
                key: KEY_DEFINDEX,
                error,
            })?;
        let quality = parse_enum_u32(KEY_QUALITY, quality_str)?;
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
    // let mut split_at = element.len();
    let bytes = element.as_bytes();
    let mut split_at = bytes.len();
    
    if split_at == 0 {
        return Ok(());
    }
    
    // Walk back through chars until a non-digit is found
    while split_at > 0 && bytes[split_at - 1].is_ascii_digit() {
        split_at -= 1;
    }
    
    // Split at where the numeric value begins (`value` will be an empty string if no digit was
    // found). This shouldn't cause issues with strings that contain varying byte lengths. If the
    // last character is multi-byte it is not a valid ascii digit, so it will stop immediately and
    // `split_at` will be the total byte length of the string.
    let (name, value) = element.split_at(split_at);
    
    match name {
        "u" => parsed.particle = Some(parse_u32(KEY_PARTICLE, value)?),
        "w" => parsed.wear = Some(parse_enum_u32(KEY_WEAR, value)?),
        "n" => parsed.craft_number = Some(parse_u32(KEY_CRAFT_NUMBER, value)?),
        "c" => parsed.crate_number = Some(parse_u32(KEY_CRATE_NUMBER, value)?),
        "p" => parsed.paint = Some(parse_enum_u32(KEY_PAINT, value)?),
        "pk" => parsed.skin = Some(parse_u32(KEY_SKIN, value)?),
        "kt-" => parsed.killstreak_tier = Some(parse_enum_u32(KEY_KILLSTREAK_TIER, value)?),
        "td-" => parsed.target_defindex = Some(parse_u32(KEY_TARGET_DEFINDEX, value)?),
        "od-" => parsed.output_defindex = Some(parse_u32(KEY_OUTPUT_DEFINDEX, value)?),
        "oq-" => parsed.output_quality = Some(parse_enum_u32(KEY_OUTPUT_QUALITY, value)?),
        "ks-" => parsed.sheen = Some(parse_enum_u32(KEY_SHEEN, value)?),
        "ke-" => parsed.killstreaker = Some(parse_enum_u32(KEY_KILLSTREAKER, value)?),
        "sp-" => {
            parsed.strange_parts.insert(parse_enum_u32(KEY_STRANGE_PART, value)?);
        },
        "footprints-" => {
            let spell = parse_enum_u32::<FootprintsSpell>(KEY_FOOTPRINTS_SPELL, value)?;
            
            parsed.spells.insert(spell.into());
        },
        "paintspell-" => {
            let spell = parse_enum_u32::<PaintSpell>(KEY_PAINT_SPELL, value)?;
            
            parsed.spells.insert(spell.into());
        },
        "voices" => {
            parsed.spells.insert(Spell::VoicesFromBelow);
        },
        "exorcism" => {
            parsed.spells.insert(Spell::Exorcism);
        },
        "halloweenfire" => {
            parsed.spells.insert(Spell::HalloweenFire);
        },
        "pumpkinbombs" => {
            parsed.spells.insert(Spell::PumpkinBombs);
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
        serializer.collect_str(self)
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
        assert_eq!(sku.quality, Quality::Normal);
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
    
    #[test]
    fn big_sku() {
        let sku = SKU {
            defindex: i32::MAX,
            quality: Quality::Strange,
            particle: Some(u32::MAX),
            skin: Some(u32::MAX),
            killstreak_tier: Some(KillstreakTier::Professional),
            wear: Some(Wear::FieldTested),
            sheen: Some(Sheen::TeamShine),
            killstreaker: Some(Killstreaker::HypnoBeam),
            craft_number: Some(u32::MAX),
            crate_number: Some(u32::MAX),
            paint: Some(Paint::DrablyOlive),
            spells: SpellSet::double(
                Spell::TeamSpiritFootprints, 
                Spell::ChromaticCorruption,
            ),
            strange_parts: StrangePartSet::triple(
                StrangePart::SappersRemoved,
                StrangePart::CloakedSpiesKilled,
                StrangePart::BuildingsDestroyed,
            ),
            craftable: false,
            australium: true,
            strange: true,
            festivized: true,
            target_defindex: Some(u32::MAX),
            output_defindex: Some(u32::MAX),
            output_quality: Some(Quality::Collectors),
        };
        
        assert!(sku.to_string().len() > 200);
        assert!(sku.to_string().len() < 250);
    }
    
    #[test]
    fn serializes() {
        let sku = "16310;15;u703;w2;pk310".parse::<SKU>().unwrap();
        
        assert_eq!(serde_json::to_string(&sku).unwrap(), r#""16310;15;u703;w2;pk310""#);
    }
}
