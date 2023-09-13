use nalgebra::Complex;

// Import the pre-generated lookup values
include!(concat!(env!("OUT_DIR"), "/cme_values.rs"));

/// CME parameter that will be generated using build.rs.
///
/// Name can't be changed.
struct CmeParam {
    pub n: usize,
    pub a: &'static [f64],
    pub b: &'static [f64],
    pub c: f64,
    pub omega: f64,
    pub mu1: f64,
    pub cv2: f64,
}

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
    // Find the steepest CME satisfying N
    let mut steepest = &CME_PARAMS[0];
    for param in CME_PARAMS.iter().skip(1) {
        if param.cv2 < steepest.cv2 && param.n < max_function_evals {
            steepest = param;
        }
    }

    let eta = std::iter::once((steepest.c * steepest.mu1).into()).chain(
        steepest
            .a
            .iter()
            .zip(steepest.b.iter())
            .map(|(a, b)| Complex::new(steepest.mu1 * a, steepest.mu1 * b)),
    );
    let beta = std::iter::once(steepest.mu1.into()).chain((0..steepest.n).map(|i| {
        Complex::new(
            steepest.mu1,
            steepest.mu1 * (i as f64 + 1.0) * steepest.omega,
        )
    }));

    // Compute inverse Laplace
    eta.zip(beta)
        .map(|(eta, beta): (Complex<f64>, Complex<f64>)| (eta * laplace_func(beta / time)).re)
        .sum::<f64>()
        / time
}
