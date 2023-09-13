use nalgebra::{Complex, ComplexField};
use pyo3::{
    types::{IntoPyDict, PyList, PyModule},
    PyResult, Python,
};

#[test]
fn staircase() {
    let x_values = [1.0, 2.0];
    let max_fn_evals = 100;

    // First get the values from the original python source code
    let python_values = run_python("staircase", &x_values, max_fn_evals);

    // Then get the values from our implementation
    run_rust(
        |s| (1.0 / s) * (1.0 / s.exp2() - 1.0),
        &x_values,
        max_fn_evals,
    );
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
                include_str!("../src/iltcme.json"),
            ))
            .map_err(|e| e.print_and_set_sys_last_vars(py))
            .unwrap()
            .extract()
            .map_err(|e| e.print_and_set_sys_last_vars(py))
            .unwrap()
    })
}
