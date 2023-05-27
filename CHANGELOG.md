# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2023-05-27

### Added

- Added `none_arr!` to create `[None; N]` for any `Option<T>`.
- Added `none_cell_arr!` to create `[None; N]` for any `None<Option<T>>`.
- Added `none_refcell_arr!` to create `[None; N]` for any `RefCell<Option<T>>`.

### Internal

- 🎉 Initial release.

[0.1.0]: https://github.com/sunsided/default-option-arr/releases/tag/0.1.0
