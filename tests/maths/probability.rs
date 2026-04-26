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

fn scalar(o: RunOutput) -> f64 {
    match o {
        RunOutput::Scalar(v) => v,
        _ => panic!("expected Scalar, got {o:?}"),
    }
}

#[test]
fn prob_normal_pdf() {
    let v = scalar(run_maths(
        "prob_normal_pdf",
        vec![
            ("x", ParameterValue::Scalar(0.0)),
            ("mu", ParameterValue::Scalar(0.0)),
            ("sigma", ParameterValue::Scalar(1.0)),
        ],
    ));
    let expected = 1.0 / (2.0 * std::f64::consts::PI).sqrt();
    assert!((v - expected).abs() < 1e-6, "N(0;0,1)≈0.3989, got {v}");
}

#[test]
fn prob_exponential_cdf() {
    let v = scalar(run_maths(
        "prob_exponential_cdf",
        vec![
            ("x", ParameterValue::Scalar(0.0)),
            ("lambda", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(v.abs() < 1e-8, "CDF(0)=0, got {v}");
}

#[test]
fn monte_carlo_pi() {
    let v = scalar(run_maths(
        "monte_carlo_pi",
        vec![
            ("n", ParameterValue::Integer(100000)),
            ("seed", ParameterValue::Integer(42)),
        ],
    ));
    assert!((v - std::f64::consts::PI).abs() < 0.1, "pi≈3.14, got {v}");
}
