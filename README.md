# tf2-sku

For parsing attributes from SKU strings.

## Usage

```rs
use tf2_sku::SKU;
use tf2_sku::tf2_enum::{Quality, KillstreakTier, Spell};

let sku = SKU::try_from("264;11;kt-3").unwrap();

assert_eq!(sku.defindex, 264);
assert_eq!(sku.quality, Quality::Strange);
assert_eq!(sku.killstreak_tier, Some(KillstreakTier::Professional));
assert_eq!(sku.to_string(), "264;11;kt-3");

// Also supports spells and strange parts
let sku = SKU::try_from("627;6;footprints-2").unwrap();

assert!(sku.spells.contains(&Spell::HeadlessHorseshoes));
```

## License

[MIT](https://github.com/juliarose/tf2-sku/blob/master/LICENSE)