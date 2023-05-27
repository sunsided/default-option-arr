# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Added `none_arr!` to create `[None; N]` for any `Option<T>`.
- Added `none_cell_arr!` to create `[None; N]` for any `None<Option<T>>`.
- Added `none_refcell_arr!` to create `[None; N]` for any `RefCell<Option<T>>`.

### Internal

- 🎉 Initial release.

<!-- [0.0.0]: https://github.com/sunsided/arraysetcell/releases/tag/0.0.0 -->
