# Changelog

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