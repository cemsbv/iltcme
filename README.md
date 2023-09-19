[![Crates.io][ci]][cl] ![MIT/Apache][li] [![docs.rs][di]][dl] ![ci][bci]

[ci]: https://img.shields.io/crates/v/iltcme.svg
[cl]: https://crates.io/crates/iltcme/
[li]: https://img.shields.io/crates/l/iltcme.svg?maxAge=2592000
[di]: https://docs.rs/iltcme/badge.svg
[dl]: https://docs.rs/iltcme/
[bci]: https://github.com/cemsbv/iltcme/workflows/ci/badge.svg

Rust implementation of Inverse Laplace Transform with Concentrated
Matrix-Exponential Functions.

Source: [https://inverselaplace.org](https://inverselaplace.org)

# Usage

Approximate a sine function where $x = 1$ with a maximum of 50 function
evaluations.

The Laplace transform of sine is $h^*(s) = 1 / (s^2 + 1)$.

```rust
fn main() {
  let result = iltcme::laplace_inversion(|s| 1.0 / (s.powi(2) + 1.0), 1.0, 50);
  approx::relative_eq!(result, 1.0_f64.sin(), epsilon = 0.001);
}
```

# Implementation details

This crate parses a large list of precomputed parameters from a JSON file and
converts them to a Rust file which is internally used. The effect of this is
that the build time might be slow, no runtime penalty should be paid though.

# Generate manually

To regenerate the Rust coefficient files run the following commands in the root:

```sh
cargo run -p gen-coefficients -- --input iltcme.json --output src/coefficients.rs
```
