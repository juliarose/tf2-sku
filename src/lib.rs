pub use tf2_enum;

use tf2_enum::{Quality, KillstreakTier, Wear, Paint, Sheen, Killstreaker};
use thiserror::Error;
use std::num::ParseIntError;
use std::convert::TryFrom;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("{}", .0)]
    ParseInt(#[from] ParseIntError),
    #[error("Invalid SKU format")]
    InvalidFormat,
    #[error("Invalid value: {}", .0)]
    InvalidValue(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MarketplaceSKU {
    pub defindex: u32,
    pub quality: Quality,
    pub australium: bool,
    pub craftable: bool,
    pub strange: bool,
    pub festivized: bool,
    pub particle: Option<u32>,
    pub killstreak_tier: Option<KillstreakTier>,
    pub wear: Option<Wear>,
    pub skin: Option<u32>,
    pub craft_number: Option<u32>,
    pub crate_number: Option<u32>,
    pub target_defindex: Option<u32>,
    pub output_defindex: Option<u32>,
    pub output_quality: Option<Quality>,
    pub paint: Option<Paint>,
    pub sheen: Option<Sheen>,
    pub killstreaker: Option<Killstreaker>,
}

impl MarketplaceSKU {
    
    fn new(defindex: u32, quality: Quality) -> Self {
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
            craft_number: None,
            crate_number: None,
            target_defindex: None,
            output_defindex: None,
            output_quality: None,
            paint: None,
            sheen: None,
            killstreaker: None,
        }
    }
}

impl TryFrom<&str> for MarketplaceSKU {
    type Error = ParseError;
    
    fn try_from(sku: &str) -> Result<Self, Self::Error> {
        let mut sku_split = sku.split(';');
        let defindex_str = sku_split.next().ok_or(ParseError::InvalidFormat)?;
        let defindex = defindex_str.parse::<u32>()?;
        let quality_str = sku_split.next().ok_or(ParseError::InvalidFormat)?;
        let quality = parse_enum_u8::<Quality>(quality_str)?;
        let mut parsed = MarketplaceSKU::new(defindex, quality);
        
        for element in sku_split {
            parse_sku_element(&mut parsed, element)?;
        }
        
        Ok(parsed)
    }
}

fn parse_sku_element(parsed: &mut MarketplaceSKU, element: &str) -> Result<(), ParseError> {
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
        "kt-" => parsed.killstreak_tier = Some(parse_enum_u8::<KillstreakTier>(value)?),
        "uncraftable" => parsed.craftable = false,
        "australium" => parsed.australium = true,
        "strange" => parsed.strange = true,
        "festive" => parsed.festivized = true,
        "w" => parsed.wear = Some(parse_enum_u8::<Wear>(value)?),
        "pk" => parsed.skin = Some(value.parse::<u32>()?),
        "n" => parsed.craft_number = Some(value.parse::<u32>()?),
        "c" => parsed.crate_number = Some(value.parse::<u32>()?),
        "td-" => parsed.target_defindex = Some(value.parse::<u32>()?),
        "od-" => parsed.output_defindex = Some(value.parse::<u32>()?),
        "oq-" => parsed.output_quality = Some(parse_enum_u8::<Quality>(value)?),
        "ks-" => parsed.sheen = Some(parse_enum_u8::<Sheen>(value)?),
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

fn parse_enum_u8<T>(s: &str) -> Result<T, ParseError>
where T:
    TryFrom<u8> + std::fmt::Display,
    <T as TryFrom<u8>>::Error: ToString,
{
    let parsed = s.parse::<u8>()?;
    let value = T::try_from(parsed)
        .map_err(|e| ParseError::InvalidValue(e.to_string()))?;
    
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn golden_frying_pan_correct() {
        assert_eq!(MarketplaceSKU::try_from("1071;11;kt-3").unwrap(), MarketplaceSKU {
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
        let sku = MarketplaceSKU::try_from("424;15;u703;w3;pk307;kt-3;ks-1;ke-2008").unwrap();
        
        assert_eq!(sku.killstreaker, Some(Killstreaker::HypnoBeam));
        assert_eq!(sku.sheen, Some(Sheen::TeamShine));
    }
    
    #[test]
    fn bad_quality_is_err() {
        assert!(MarketplaceSKU::try_from("1071;122").is_err());
    }
    
    #[test]
    fn negative_defindex_is_err() {
        assert!(MarketplaceSKU::try_from("-1;11").is_err());
    }
    
    #[test]
    fn paint_kit_correct() {
        assert!(MarketplaceSKU::try_from("16310;15;u703;w2;pk310").is_ok());
    }
}