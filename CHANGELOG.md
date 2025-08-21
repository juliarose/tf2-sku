# Changelog

### 0.6.0 (2025-08-21)

### Added
- `BitAnd` for `SpellSet` and `StrangePartSet`, and their borrowed variants.
- `is_subset` and `is_superset` methods to `StrangePartSet` and `SpellSet`.
- `impl std::error::Error` for `ParseError`.

### Changed
- Moved `StrangePartSet` and `SpellSet` into `tf2_enum`.
- Bumped `tf2-enum` to `^0.11.0`.
- Faster serialization by using `collect_str`.
- Failed quality parsing for `SKU::parse_attributes` now defaults to `Quality::Normal`.

## 0.5.0 (2024-05-29)

### Added
- `InsertError` for errors when inserting to `StrangePartSet` or `SpellSet`.
- `single` method for `StrangePartSet` and `SpellSet` to create a new collection with a single element.
- `double` method for `StrangePartSet` and `SpellSet` to create a new collection with two elements.
- `triple` method for `StrangePartSet` to create a new collection with three elements.
- `take`, `difference`, `intersection`, `is_disjoint` methods for `StrangePartSet` and `SpellSet`.
- `TryFrom<String>` and `TryFrom<&String>` for `SKU`.

### Changed
- `StrangeParts` to `StrangePartSet`.
- `Spells` to `SpellSet`.
- `StrangePartSet` and `SpellSet` now return `InsertError` when an insert fails instead of silently ignoring the insert.
- `StrangePartSet` and `SpellSet` new methods no longer take an argument.

## 0.4.2 (2024-05-14)

### Fixed
- Minor issue with `parse_enum_u32` in `helpers`.

## 0.4.1 (2024-02-09)

### Added
- More documentation.

## 0.4.0 (2023-10-06)

### Added
- `StrangeParts` to `SKU`.
- `Spells` to `SKU`.

### Changed
- Performance enhancements with `fmt::Display` impl of `SKU`.
- Moved `ParseError` into `error` module.

## 0.3.0 (2023-02-24)

### Added
- Borrowed implementation of `SKUString` for `SKU`.
- `parse_attributes` method to `SKU`.
- `std::str::FromStr` impl to `SKU`.

### Changed
- Bump `tf2-enum` to `0.8.0`.
- Better error types/descriptions.

### Removed
- `string_concat` dependency.

## 0.2.0 (2022-10-14)

### Changed
- Bump `tf2-enum` to `0.6.0`.

## 0.1.0 (2022-04-28)