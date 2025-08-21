//! SKU parser for Team Fortress 2 items.
//! 
//! ## Usage
//! ```
//! use tf2_sku::SKU;
//! use tf2_enum::prelude::*;
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
mod sku;

pub use sku::{SKU, SKUString};
pub use tf2_enum;
