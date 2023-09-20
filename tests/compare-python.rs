use nalgebra::{Complex, ComplexField};
use pyo3::{
    types::{PyList, PyModule},
    Python,
};

/// How much worse the Rust is allowed to be compared to the Python result.
const ALLOWED_ERROR: f64 = 1e-8;

/// Values to test.
const TEST_VALUES: [f64; 13] = [
    1.0, 1e-3, 1e-5, 1e-8, 1e-15, 1e+3, 1e+5, 1e+8, 1e+15, 3.0, -1.0, -1e-8, -1e+8,
];

#[test]
fn exponential() {
    for max_fn_evals in 1..500 {
        compare_values(
            &TEST_VALUES,
            |x| (-x).exp(),
            "exponential",
            |s| (s + 1.0).recip(),
            max_fn_evals,
        );
    }
}

#[test]
fn sine() {
    for max_fn_evals in 1..500 {
        compare_values(
            &TEST_VALUES,
            |x| x.sin(),
            "sine",
            |s| (1.0 + s.powi(2)).recip(),
            max_fn_evals,
        );
    }
}

#[test]
fn squarewave() {
    for max_fn_evals in 1..500 {
        compare_values(
            &[1.0, 2.0, 2.5, 3.0, 4.1, 5.01],
            |x| x.floor() % 2.0,
            "squarewave",
            |s| s.recip() * (1.0 + s.exp()).recip(),
            max_fn_evals,
        );
    }
}

#[test]
fn staircase() {
    for max_fn_evals in 1..500 {
        compare_values(
            &[1.0, 2.0, 2.5, 3.0, 4.1, 5.01],
            |x| x.floor(),
            "staircase",
            |s| s.recip() * (s.exp() - 1.0).recip(),
            max_fn_evals,
        );
    }
}

fn compare_values(
    x_values: &[f64],
    expected_fn: impl Fn(f64) -> f64,
    python_func_name: &str,
    rust_func: impl Fn(Complex<f64>) -> Complex<f64>,
    max_fn_evals: usize,
) {
    // First get the values from the original python source code
    let python_values = run_python(python_func_name, x_values, max_fn_evals);

    // Then get the values from our implementation
    let rust_values = run_rust(rust_func, x_values, max_fn_evals);

    x_values
        .iter()
        .zip(python_values)
        .zip(rust_values)
        .for_each(|((x, python), rust)| {
            let real = expected_fn(*x);

            // Ensure that our Rust value is closer or equal to the Python solution
            let python_dist = (real - python).abs();
            let rust_dist = (real - rust).abs();
            assert!(rust_dist <= python_dist + ALLOWED_ERROR, "Rust result is worse than original Python result:\n\tPython Delta: {python_dist}\n\tRust Delta  : {rust_dist}\n\tPython      : {python}\n\tRust        : {rust}\n\tExpected    : {real}\n\tFn evals    : {max_fn_evals}\n\tX           : {x}");
        })
}

fn run_rust(
    lt_func: impl Fn(Complex<f64>) -> Complex<f64>,
    test_values: &[f64],
    max_fn_evals: usize,
) -> Vec<f64> {
    test_values
        .iter()
        .map(|test_value| iltcme::laplace_inversion(&lt_func, *test_value, max_fn_evals))
        .collect()
}

fn run_python(lt_func: &str, test_values: &[f64], max_fn_evals: usize) -> Vec<f64> {
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        // Load 'tests/iltcme.py'
        let py_mod = PyModule::from_code(py, include_str!("./iltcme.py"), "iltcme.py", "iltcme")
            .map_err(|e| e.print_and_set_sys_last_vars(py))
            .unwrap();

        // Get the 'ilt' function from the 'iltcme' module
        let py_fun = py_mod
            .getattr("ilt")
            .map_err(|e| e.print_and_set_sys_last_vars(py))
            .unwrap();

        // Call 'ilt'
        py_fun
            .call1((
                lt_func,
                PyList::new(py, test_values.iter()),
                max_fn_evals,
                include_str!("../iltcme.json"),
            ))
            .map_err(|e| e.print_and_set_sys_last_vars(py))
            .unwrap()
            .extract()
            .map_err(|e| e.print_and_set_sys_last_vars(py))
            .unwrap()
    })
}
