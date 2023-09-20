//! Generate a Rust source file with all parameters as proper structs.

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use clap::Parser;
use serde::Deserialize;
use serde_json::value::RawValue;

/// Convert coefficients from JSON to Rust files.
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// JSON coefficients file path.
    #[arg(short, long)]
    pub input: PathBuf,
    /// Output Rust source code file path.
    #[arg(short, long)]
    pub output: PathBuf,
    /// Max evaluations to calculate.
    #[arg(short, long, default_value_t = 500)]
    pub max_evaluations: usize,
    /// Export the raw coefficients instead of precalculated values.
    #[arg(short, long)]
    pub raw: bool,
}

#[derive(Debug, Deserialize)]
struct ParsedParam {
    pub n: usize,
    pub a: Vec<f64>,
    pub b: Vec<f64>,
    pub c: f64,
    pub omega: f64,
    pub mu1: f64,
    pub cv2: f64,
}

#[derive(Debug, Deserialize)]
struct RawParam<'a> {
    pub n: usize,
    #[serde(borrow)]
    pub a: Vec<&'a RawValue>,
    #[serde(borrow)]
    pub b: Vec<&'a RawValue>,
    #[serde(borrow)]
    pub c: &'a RawValue,
    #[serde(borrow)]
    pub omega: &'a RawValue,
    #[serde(borrow)]
    pub mu1: &'a RawValue,
    #[serde(borrow)]
    pub cv2: &'a RawValue,
}

/// Convert all ILTCME values to eta and beta complex pairs.
fn generate_precomputed<W>(json: &str, out: &mut BufWriter<W>, max_evaluations: usize)
where
    W: Write,
{
    // Read the json file
    let params: Vec<ParsedParam> = serde_json::from_str(json).unwrap();

    // Re-export the maximum function evaluations
    writeln!(
        out,
        "pub(crate) const MAX_EVALUATIONS: usize = {max_evaluations};"
    )
    .unwrap();
    // Create a lookup list for each iteration
    write!(
        out,
        "pub(crate) const ETA_BETA_PAIRS: [(f64, &[(f64, f64, f64)], f64); {max_evaluations}] = ["
    )
    .unwrap();

    // Calculate the etas and betas for each maximum of function evaluations
    let mut consts = String::new();
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
        let beta = (0..steepest.n).map(|i| ((i + 1) as f64) * steepest.omega * steepest.mu1);

        let eta_betas = eta.zip(beta).collect::<Vec<_>>();
        consts += &format!(
            "const E{index:X}:[(f64,f64,f64);{}]={};\n",
            eta_betas.len(),
            fmt_vec(&eta_betas)
        );
        write!(
            out,
            "({},&E{index:X},{}),",
            fmt_f64(steepest.mu1),
            fmt_f64(steepest.c * steepest.mu1),
        )
        .unwrap();
    });

    writeln!(out, "];\n{consts}").unwrap();
}

fn fmt_vec(v: &[((f64, f64), f64)]) -> String {
    format!(
        "[{}]",
        v.iter()
            .map(|((v1, v2), v3)| format!("({},{},{})", fmt_f64(*v1), fmt_f64(*v2), fmt_f64(*v3)))
            .collect::<Vec<String>>()
            .join(",")
    )
}

/// Properly print floats so Rust source code can parse them.
fn fmt_f64(v: f64) -> String {
    if v.fract() == 0.0 {
        // Always print as a float
        format!("{v}.")
    } else {
        format!("{v}")
    }
}

/// Only convert the ILTCME values to Rust.
fn generate_raw<W>(json: &str, out: &mut BufWriter<W>)
where
    W: Write,
{
    // Read the json file
    let params: Vec<RawParam> = serde_json::from_str(json).unwrap();

    // Create the data arrays
    params
        .iter()
        .enumerate()
        .for_each(|(i, RawParam { a, b, .. })| {
            write!(out, "const A_{i}: [f64; {}] = ", a.len()).unwrap();
            write_raw_vec(out, a);
            write!(out, ";\nconst B_{i}: [f64; {}] = ", b.len()).unwrap();
            write_raw_vec(out, b);
            writeln!(out, ";").unwrap();
        });

    // Create the parameters
    write!(
        out,
        "pub(crate) const CME_PARAMS: [CmeParam; {}] = [",
        params.len()
    )
    .unwrap();
    params.into_iter().enumerate().for_each(| (i, RawParam { n, c, omega,  mu1, cv2, .. })| {
            writeln!(out, "CmeParam {{ n: {n}, a: &A_{i}, b: &B_{i}, c: {c}, omega: {omega}, mu1: {mu1}, cv2: {cv2} }},").unwrap();
        });
    writeln!(out, "];").unwrap();
}

fn write_raw_vec(s: &mut impl Write, v: &[&RawValue]) {
    write!(s, "[").unwrap();
    v.iter().for_each(|v| write!(s, "{v},").unwrap());
    write!(s, "]").unwrap();
}

fn main() {
    let args = Args::parse();

    let file = File::create(args.output).unwrap();
    let mut out = BufWriter::new(file);

    writeln!(out, "//! Auto-generated coefficient file, don't edit.\n").unwrap();
    writeln!(out, "#![cfg_attr(rustfmt, rustfmt_skip)]").unwrap();
    writeln!(out, "#[allow(clippy::all)]").unwrap();

    let json = std::fs::read_to_string(args.input).unwrap();
    if args.raw {
        generate_raw(&json, &mut out);
    } else {
        generate_precomputed(&json, &mut out, args.max_evaluations);
    }
}
