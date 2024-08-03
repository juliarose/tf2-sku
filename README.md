# tf2-sku

For parsing attributes from SKU strings.

## Usage

```rust
use tf2_sku::SKU;
use tf2_enum::{Quality, KillstreakTier, Spell, StrangePart};

let sku = "264;11;kt-1".parse::<SKU>().unwrap();

assert_eq!(sku.defindex, 264);
assert_eq!(sku.quality, Quality::Strange);
assert_eq!(sku.killstreak_tier, Some(KillstreakTier::Killstreak));
assert_eq!(sku.to_string(), "264;11;kt-1");

// Also supports spells and strange parts
let sku = "627;11;footprints-2;sp-28".parse::<SKU>().unwrap();

assert!(sku.spells.contains(&Spell::HeadlessHorseshoes));
assert!(sku.strange_parts.contains(&StrangePart::Dominations));
```

## License

[MIT](https://github.com/juliarose/tf2-sku/blob/master/LICENSE)