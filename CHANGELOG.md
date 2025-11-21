# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0] - 2025-11-21

### Performance

- *(clippy)* Properly ignore clippy lints with generated coefficient file

### Refactor

- *(complex)* [**breaking**] Replace heavy `nalgebra` crate with `num-complex` for the `Complex<f64>` type

## [0.2.3] - 2025-09-09

### Bug Fixes

- *(deps)* Update all dependencies
- *(deps)* Update all non-major dependencies
- *(deps)* Update all non-major dependencies

### Miscellaneous Tasks

- *(ci)* Move from dependabot to renovate

## [0.2.2] - 2024-01-31

### Features

- Add `laplace_inversion_mut` which accepts a `FnMut` closure

## [0.2.1] - 2024-01-29

### Miscellaneous Tasks

- Formatting

## [0.2.0] - 2023-09-20

### Bug Fixes

- Improve numerical accuracy

### Miscellaneous Tasks

- Release
- Release

### Refactor

- Change maximum evaluations to 500
- Move build step to separate CLI
- Add advanced pre-computations (#1)

## [0.1.0] - 2023-09-01

### Miscellaneous Tasks

- Initial commit

<!-- CEMS BV. -->
