//! Generate a Rust source file with all parameters as proper structs.

/// Convert all ILTCME values to eta and beta complex pairs.
#[cfg(feature = "precomputed")]
mod generate {
    use std::{fmt::Display, path::Path};

    use nalgebra::Complex;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum ValOrArray {
        Val(f64),
        Vec(Vec<f64>),
    }

    #[derive(Debug, Deserialize)]
    struct Param {
        pub n: usize,
        pub a: Vec<f64>,
        pub b: Vec<f64>,
        pub c: f64,
        pub omega: f64,
        pub mu1: f64,
        pub cv2: f64,
    }

    pub fn generate() {
        // Get the maximum amount of parameters we want to pre-process for check from the env var
        let max_evaluations: usize = std::env::var("ILTCME_MAX_EVALUATIONS")
            .unwrap_or("1000".to_string())
            .parse()
            .unwrap();

        // Read the json file
        let params: Vec<Param> = serde_json::from_str(include_str!("src/iltcme.json")).unwrap();

        // Create the array string
        let mut s = String::new();
        let mut consts = String::new();
        // Re-export the maximum function evaluations
        s += &format!("const MAX_EVALUATIONS: usize = {max_evaluations};\n");
        // Create a lookup list for each iteration
        s += &format!(
        "const ETA_BETA_PAIRS: [(f64, &'static [(f64, f64, f64)], f64, f64); {max_evaluations}] = ["
    );

        // Calculate the etas and betas for each maximum of function evaluations
        (0..max_evaluations).for_each(|index| {
            // Find the steepest CME satisfying N
            let mut steepest = &params[0];
            for param in params.iter().skip(1).filter(|param| param.n < index) {
                if param.cv2 < steepest.cv2 {
                    steepest = param;
                }
            }

            let eta = steepest
                .a
                .iter()
                .zip(steepest.b.iter())
                .map(|(a, b)| (steepest.mu1 * a, steepest.mu1 * b));
            let beta = (0..steepest.n).map(|i| steepest.mu1 * (i as f64 + 1.0) * steepest.omega);

            let eta_betas = eta.zip(beta).collect::<Vec<_>>();
            consts += &format!(
                "const ETA_BETA_PAIRS_{index}: [(f64, f64, f64); {}] = {};\n",
                eta_betas.len(),
                fmt_vec(&eta_betas)
            );
            s += &format!(
                "({},&ETA_BETA_PAIRS_{index},{}f64,{}f64),",
                steepest.mu1,
                steepest.c * steepest.mu1,
                steepest.mu1
            );
        });

        s += "];";

        // Write the params to a file
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("cme_values.rs");
        std::fs::write(dest_path, format!("{consts}\n{s}")).unwrap();
    }

    fn fmt_vec(v: &[((f64, f64), f64)]) -> String {
        format!(
            "[{}]",
            v.iter()
                .map(|((v1, v2), v3)| format!("({}f64,{}f64,{}f64)", v1, v2, v3))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

/// Only convert the ILTCME values to Rust.
#[cfg(not(feature = "precomputed"))]
mod generate {
    use std::{fmt::Display, path::Path};

    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum ValOrArray {
        Val(f64),
        Vec(Vec<f64>),
    }

    impl Display for ValOrArray {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ValOrArray::Val(v) => f.write_str(&format!("[{v}]")),
                ValOrArray::Vec(v) => f.write_str(&fmt_vec(v)),
            }
        }
    }

    fn fmt_vec(v: &[f64]) -> String {
        format!(
            "[{}]",
            v.iter()
                .map(|v| format!("{v}"))
                .collect::<Vec<String>>()
                .join(",")
        )
    }

    #[derive(Debug, Deserialize)]
    struct Param {
        pub n: usize,
        pub a: Vec<f64>,
        pub b: Vec<f64>,
        pub c: f64,
        pub omega: f64,
        pub mu1: f64,
        pub cv2: f64,
    }

    pub fn generate() {
        // Rebuild if math changes
        println!("cargo:rerun-if-changed=src/iltcme.json");

        // Read the json file
        let params: Vec<Param> = serde_json::from_str(include_str!("src/iltcme.json")).unwrap();

        // Create the array string
        let mut s = String::new();
        let mut consts = String::new();
        s += &format!("const CME_PARAMS: [CmeParam; {}] = [", params.len());

        s += &params.into_iter().enumerate().map(| (i, Param { n, a, b, c, omega,  mu1, cv2 })| {
        consts += &format!("const A_{i}: [f64; {}] = {};\n", a.len(), fmt_vec(&a));
        consts += &format!("const B_{i}: [f64; {}] = {};\n", b.len(), fmt_vec(&b));

        format!("CmeParam {{ n: {n}, a: &A_{i}, b: &B_{i}, c: {c}, omega: {omega}, mu1: {mu1}, cv2: {cv2} }}")
    }).collect::<Vec<String>>().join(",\n");

        s += "];";

        // Write the params to a file
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("cme_values.rs");
        std::fs::write(dest_path, format!("{consts}\n{s}")).unwrap();
    }
}

fn main() {
    // Rebuild if math changes
    println!("cargo:rerun-if-changed=src/iltcme.json");

    generate::generate();
}
