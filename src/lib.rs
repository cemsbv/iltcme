#![doc = include_str!("../README.md")]

#[cfg(not(feature = "precomputed"))]
mod computed;
#[cfg(feature = "precomputed")]
mod precomputed;

#[cfg(not(feature = "precomputed"))]
pub use computed::laplace_inversion;
#[cfg(feature = "precomputed")]
pub use precomputed::laplace_inversion;

#[cfg(test)]
mod tests {
    use nalgebra::{Complex, ComplexField};

    use super::*;

    /// Calculate and compare the inversion of the different laplace function for a range of numbers.
    fn invert_fns(max_function_evals: usize) {
        invert_fn(
            "Exponential",
            |s| (1.0 + s).recip(),
            |s| (-s).exp(),
            &[0.01, 0.1, 1.0, 10.0],
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
