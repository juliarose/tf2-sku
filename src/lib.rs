pub use tf2_enum;

use tf2_enum::{Quality, KillstreakTier, Wear, Paint, Sheen, Killstreaker};
use thiserror::Error;
use std::fmt;
use std::num::ParseIntError;
use std::convert::TryFrom;
use serde::{Serialize, Serializer, de::{self, Visitor}};

/// Attributes related to a SKU string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SKU {
    /// This can be expected to be negative.
    pub defindex: i32,
    pub quality: Quality,
    pub australium: bool,
    pub craftable: bool,
    pub strange: bool,
    pub festivized: bool,
    pub particle: Option<u32>,
    pub killstreak_tier: Option<KillstreakTier>,
    pub wear: Option<Wear>,
    pub skin: Option<u32>,
    pub target_defindex: Option<u32>,
    pub output_defindex: Option<u32>,
    pub output_quality: Option<Quality>,
    pub craft_number: Option<u32>,
    pub crate_number: Option<u32>,
    pub paint: Option<Paint>,
    pub sheen: Option<Sheen>,
    pub killstreaker: Option<Killstreaker>,
}

impl SKU {
    
    /// Creates a new SKU using the given defindex and quality. All other fields will be `None` or 
    /// `false` with the exception of craftable, which is `true`. 
    /// 
    /// # Examples
    ///
    /// ```
    /// use tf2_sku::{SKU, tf2_enum::Quality};
    /// 
    /// SKU::new(264, Quality::Strange);
    /// ```
    pub fn new(
        defindex: i32,
        quality: Quality,
    ) -> Self {
        Self {
            defindex,
            quality,
            australium: false,
            craftable: true,
            strange: false,
            festivized: false,
            particle: None,
            killstreak_tier: None,
            wear: None,
            skin: None,
            target_defindex: None,
            output_defindex: None,
            output_quality: None,
            craft_number: None,
            crate_number: None,
            paint: None,
            sheen: None,
            killstreaker: None,
        }
    }
    
    /// Removes attributes that do not belong to an item's base SKU. These include paint,
    /// killstreaker, and sheen.
    pub fn remove_extras(&mut self) {
        self.paint = None;
        self.sheen = None;
        self.killstreaker = None;
    }
}

/// Formats SKU attributes into a string.
/// 
/// # Examples
///
/// ```
/// use tf2_sku::{SKU, tf2_enum::{Quality, KillstreakTier}};
/// 
/// let mut sku = SKU::new(264, Quality::Strange);
/// 
/// sku.killstreak_tier = Some(KillstreakTier::Professional);
/// 
/// assert_eq!(&sku.to_string(), "264;11;kt-3");
/// ```
impl fmt::Display for SKU {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = self.defindex.to_string() + ";";
        
        string.push_str(&(self.quality as u32).to_string());
            
        if let Some(particle) = &self.particle {
            string.push_str(";u");
            string.push_str(&particle.to_string());
        }
        
        if !self.craftable {
            string.push_str(";uncraftable");
        }
        
        if self.australium {
            string.push_str(";australium");
        }
        
        if self.strange {
            string.push_str(";strange");
        }
        
        if let Some(wear) = &self.wear {
            string.push_str(";w");
            string.push_str(&(*wear as u32).to_string());
        }
        
        if let Some(skin) = &self.skin {
            string.push_str(";pk");
            string.push_str(&skin.to_string());
        }
        
        if let Some(killstreak_tier) = &self.killstreak_tier {
            string.push_str(";kt-");
            string.push_str(&(*killstreak_tier as u32).to_string());
        }
        
        if self.festivized {
            string.push_str(";festive");
        }

        if let Some(crate_number) = &self.crate_number {
            string.push_str(";c");
            string.push_str(&crate_number.to_string());
        }

        if let Some(craft_number) = &self.craft_number {
            string.push_str(";n");
            string.push_str(&craft_number.to_string());
        }

        if let Some(target_defindex) = &self.target_defindex {
            string.push_str(";td-");
            string.push_str(&target_defindex.to_string());
        }

        if let Some(output_defindex) = &self.output_defindex {
            string.push_str(";od-");
            string.push_str(&output_defindex.to_string());
        }

        if let Some(output_quality) = &self.output_quality {
            string.push_str(";oq-");
            string.push_str(&(*output_quality as u32).to_string());
        }

        if let Some(paint) = &self.paint {
            string.push_str(";p");
            string.push_str(&(*paint as u32).to_string());
        }

        if let Some(sheen) = &self.sheen {
            string.push_str(";ks-");
            string.push_str(&(*sheen as u32).to_string());
        }

        if let Some(killstreaker) = &self.killstreaker {
            string.push_str(";ke-");
            string.push_str(&(*killstreaker as u32).to_string());
        }

        write!(f, "{}", string)
    }
}

/// Attempts to parse a SKU from a string.
/// 
/// # Examples
///
/// ```
/// use tf2_sku::{SKU, tf2_enum::{Quality, KillstreakTier}};
/// 
/// let sku = SKU::try_from("264;11;kt-3").unwrap();
/// 
/// assert_eq!(sku.defindex, 264);
/// assert_eq!(sku.quality, Quality::Strange);
/// assert_eq!(sku.killstreak_tier, Some(KillstreakTier::Professional));
/// ```
impl TryFrom<&str> for SKU {
    type Error = ParseError;
    
    fn try_from(sku: &str) -> Result<Self, Self::Error> {
        let mut sku_split = sku.split(';');
        let defindex_str = sku_split.next().ok_or(ParseError::InvalidFormat)?;
        let defindex = defindex_str.parse::<i32>()?;
        let quality_str = sku_split.next().ok_or(ParseError::InvalidFormat)?;
        let quality = parse_enum_u32::<Quality>(quality_str)?;
        let mut parsed = SKU::new(defindex, quality);
        
        for element in sku_split {
            parse_sku_element(&mut parsed, element)?;
        }
        
        Ok(parsed)
    }
}

fn parse_sku_element(parsed: &mut SKU, element: &str) -> Result<(), ParseError> {
    let mut split_at = element.len();
    
    for c in element.chars().rev() {
        if c.is_digit(10) {
            split_at -= 1;
        } else {
            break;
        }
    }
    
    let (name, value) = element.split_at(split_at);
    
    match name {
        "u" => parsed.particle = Some(value.parse::<u32>()?),
        "kt-" => parsed.killstreak_tier = Some(parse_enum_u32::<KillstreakTier>(value)?),
        "uncraftable" => parsed.craftable = false,
        "australium" => parsed.australium = true,
        "strange" => parsed.strange = true,
        "festive" => parsed.festivized = true,
        "w" => parsed.wear = Some(parse_enum_u32::<Wear>(value)?),
        "pk" => parsed.skin = Some(value.parse::<u32>()?),
        "n" => parsed.craft_number = Some(value.parse::<u32>()?),
        "c" => parsed.crate_number = Some(value.parse::<u32>()?),
        "td-" => parsed.target_defindex = Some(value.parse::<u32>()?),
        "od-" => parsed.output_defindex = Some(value.parse::<u32>()?),
        "oq-" => parsed.output_quality = Some(parse_enum_u32::<Quality>(value)?),
        "ks-" => parsed.sheen = Some(parse_enum_u32::<Sheen>(value)?),
        "ke-" => parsed.killstreaker = Some(parse_enum_u32::<Killstreaker>(value)?),
        "p" => parsed.paint = Some(parse_enum_u32::<Paint>(value)?),
        _ => {},
    }
    
    Ok(())
}

fn parse_enum_u32<T>(s: &str) -> Result<T, ParseError>
where T:
    TryFrom<u32> + std::fmt::Display,
    <T as TryFrom<u32>>::Error: ToString,
{
    let parsed = s.parse::<u32>()?;
    let value = T::try_from(parsed)
        .map_err(|e| ParseError::InvalidValue(e.to_string()))?;
    
    Ok(value)
}

/// An error when parsing from a string.
#[derive(Error, Debug)]
pub enum ParseError {
    /// An integer failed to fit into the target type.
    #[error("{}", .0)]
    ParseInt(#[from] ParseIntError),
    /// The SKU format is not valid.
    #[error("Invalid SKU format")]
    InvalidFormat,
    /// An attrbiute value is not valid.
    #[error("Invalid value: {}", .0)]
    InvalidValue(String),
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

    #[derive(Serialize, Deserialize)]
    struct Item {
        sku: SKU,
    }
    
    #[test]
    fn golden_frying_pan_correct() {
        assert_eq!(SKU::try_from("1071;11;kt-3").unwrap(), SKU {
            defindex: 1071,
            quality: Quality::Strange,
            australium: false,
            craftable: true,
            strange: false,
            festivized: false,
            particle: None,
            killstreak_tier: Some(KillstreakTier::Professional),
            wear: None,
            skin: None,
            craft_number: None,
            crate_number: None,
            target_defindex: None,
            output_defindex: None,
            output_quality: None,
            sheen: None,
            killstreaker: None,
            paint: None,
        });
    }
    
    #[test]
    fn professional_killstreak_skin() {
        let sku = SKU::try_from("424;15;u703;w3;pk307;kt-3;ks-1;ke-2008").unwrap();
        
        assert_eq!(sku.killstreaker, Some(Killstreaker::HypnoBeam));
        assert_eq!(sku.sheen, Some(Sheen::TeamShine));
    }
    
    #[test]
    fn bad_quality_is_err() {
        assert!(SKU::try_from("1071;122").is_err());
    }
    
    #[test]
    fn negative_defindex_is_ok() {
        assert!(SKU::try_from("-1;11").is_ok());
    }
    
    #[test]
    fn paint_kit_correct() {
        assert!(SKU::try_from("16310;15;u703;w2;pk310").is_ok());
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
        let sku = SKU::try_from("16310;15;u703;w2;pk310").unwrap();
        let s = serde_json::to_string(&Item { sku }).unwrap();

        assert_eq!(s, r#"{"sku":"16310;15;u703;w2;pk310"}"#);
    }
}