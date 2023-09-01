#![doc = include_str!("../README.md")]

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

#[cfg(test)]
mod tests {
    use nalgebra::ComplexField;

    use super::*;

    /// Calculate and compare the inversion of the different laplace function for a range of numbers.
    fn invert_fns(max_function_evals: usize) {
        invert_fn(
            "Exponential",
            |s| (1.0 + s).recip(),
            |s| (-s).exp(),
            &[0.0001, 0.1, 1.0, 10.0],
            max_function_evals,
        );
        invert_fn(
            "Sine",
            |s| (1.0 + s.powi(2)).recip(),
            |s| s.sin(),
            &[0.1, 0.2, 1.0, 2.0, 4.0],
            max_function_evals,
        );
        invert_fn(
            "Staircase",
            |s| s.recip() / (s.exp() - 1.0),
            |s| s.floor(),
            &[0.5, 1.5, 2.5, 3.5, 4.5],
            max_function_evals,
        );
    }

    /// Calculate and compare the inversion of the laplace function for a range of numbers.
    fn invert_fn(
        name: &str,
        func: fn(time: Complex<f64>) -> Complex<f64>,
        inverse: fn(time: f64) -> f64,
        times: &[f64],
        max_function_evals: usize,
    ) {
        for time in times {
            let result = laplace_inversion(func, *time, max_function_evals);
            let compare = inverse(*time);

            assert!(
                approx::relative_eq!(result, compare, epsilon = 0.01),
                "Inversion of function \"{name}\" failed:\n\tTime     : {time}\n\tResult   : {result}\n\tCompare  : {compare}\n\tMax Evals: {max_function_evals}"
            );
        }
    }

    /// Source: https://github.com/ghorvath78/iltcme/blob/master/matlab_demo.m
    #[test]
    fn laplace_inversions() {
        invert_fns(30);
    }
}
