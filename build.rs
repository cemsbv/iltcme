//! Generate a Rust source file with all parameters as proper structs.

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

fn main() {
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
