use sciforge::hub::prelude::*;

fn run_maths(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Maths, name);
    for (k, v) in params {
        exp = exp.param(k, v);
    }
    ExperimentRunner::new()
        .run(&exp)
        .unwrap_or_else(|_| panic!("dispatch '{name}' failed"))
}

#[test]
fn legendre() {
    let o = run_maths("legendre", vec![("n", ParameterValue::Integer(0))]);
    match o {
        RunOutput::PolynomialOut(coeffs) => {
            assert!(!coeffs.is_empty(), "P0 should have coefficients");
            assert!((coeffs[0] - 1.0).abs() < 1e-8, "P0=[1], got {:?}", coeffs);
        }
        other => panic!("expected PolynomialOut, got {other:?}"),
    }
}

#[test]
fn chebyshev_t() {
    let o = run_maths("chebyshev_t", vec![("n", ParameterValue::Integer(0))]);
    match o {
        RunOutput::PolynomialOut(coeffs) => {
            assert!(!coeffs.is_empty(), "T0 should have coefficients");
            assert!((coeffs[0] - 1.0).abs() < 1e-8, "T0=[1], got {:?}", coeffs);
        }
        other => panic!("expected PolynomialOut, got {other:?}"),
    }
}

#[test]
fn polynomial_roots_real() {
    let out = run_maths(
        "polynomial_roots_real",
        vec![
            ("poly", ParameterValue::Polynomial(vec![1.0, 0.0, -1.0])),
            ("range_min", ParameterValue::Scalar(-10.0)),
            ("range_max", ParameterValue::Scalar(10.0)),
            ("subdivisions", ParameterValue::Integer(1000)),
            ("tol", ParameterValue::Scalar(1e-10)),
        ],
    );
    match out {
        RunOutput::Vector(v) => {
            assert!(!v.is_empty(), "x²-1 should have real roots");
        }
        _ => panic!("expected Vector output, got {out:?}"),
    }
}
