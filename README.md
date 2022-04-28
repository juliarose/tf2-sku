# tf2-sku

For parsing attributes from SKU strings.

## Usage

``rs
use tf2_sku::{SKU, tf2_enum::{Quality, KillstreakTier}};

let sku = SKU::try_from("264;11;kt-3").unwrap();

assert_eq!(sku.defindex, 264);
assert_eq!(sku.quality, Quality::Strange);
assert_eq!(sku.killstreak_tier, Some(KillstreakTier::Professional));
``

## License

MIT