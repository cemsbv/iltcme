#![doc = include_str!("../README.md")]

use nalgebra::Complex;

// Import the pre-generated lookup values
// Calculates the ETA_BETA_PAIRS lookup
include!(concat!(env!("OUT_DIR"), "/cme_values.rs"));

/// Calculate the Laplace inversion for a function using the CME method.
///
/// Evaluates the Laplace transform expression at certain points to approximate the inverse of the Laplace transform at a given point.
///
/// # Example
///
/// ```rust
/// # fn main() {
/// // Approximate a sine function where `x = 1`
/// // The Laplace transform of sine is `h*(s) = 1 / (s^2 + 1)`
/// let result = iltcme::laplace_inversion(|s| 1.0 / (s.powi(2) + 1.0), 1.0, 50);
/// approx::relative_eq!(result, 1.0_f64.sin(), epsilon = 0.001);
/// # }
/// ```
pub fn laplace_inversion(
    laplace_func: impl Fn(Complex<f64>) -> Complex<f64>,
    time: f64,
    max_function_evals: usize,
) -> f64 {
    assert!(max_function_evals < MAX_EVALUATIONS);

    // Compute inverse Laplace
    let (mu1, eta_betas, first_eta, first_beta) = ETA_BETA_PAIRS[max_function_evals];
    std::iter::once((first_eta.into(), first_beta.into()))
        .chain(eta_betas.iter().map(|(eta_re, eta_im, beta)| {
            (Complex::new(*eta_re, *eta_im), Complex::new(mu1, *beta))
        }))
        .map(|(eta, beta)| (eta * laplace_func(beta / time)).re)
        .sum::<f64>()
        / time
}
