use sciforge_hub::prelude::*;

fn run_chem(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Chemistry, name);
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
fn langmuir_isotherm() {
    let v = scalar(run_chem(
        "langmuir_isotherm",
        vec![
            ("theta_max", ParameterValue::Scalar(1.0)),
            ("k", ParameterValue::Scalar(0.1)),
            ("pressure", ParameterValue::Scalar(10.0)),
        ],
    ));
    let expected = 1.0 * 0.1 * 10.0 / (1.0 + 0.1 * 10.0);
    assert!(
        (v - expected).abs() < 1e-6,
        "theta=KP/(1+KP)={expected}, got {v}"
    );
}

#[test]
fn freundlich_isotherm() {
    let v = scalar(run_chem(
        "freundlich_isotherm",
        vec![
            ("kf", ParameterValue::Scalar(5.0)),
            ("pressure", ParameterValue::Scalar(1.0)),
            ("n", ParameterValue::Scalar(2.0)),
        ],
    ));
    assert!((v - 5.0).abs() < 1e-8, "q=Kf*C^(1/n)=5*1=5, got {v}");
}

#[test]
fn laplace_pressure() {
    let v = scalar(run_chem(
        "laplace_pressure",
        vec![
            ("gamma", ParameterValue::Scalar(0.072)),
            ("r1", ParameterValue::Scalar(1e-6)),
            ("r2", ParameterValue::Scalar(1e-6)),
        ],
    ));
    assert!(v > 0.0, "Laplace pressure should be positive, got {v}");
}

#[test]
fn contact_angle() {
    let v = scalar(run_chem(
        "contact_angle",
        vec![
            ("gamma_sv", ParameterValue::Scalar(0.05)),
            ("gamma_sl", ParameterValue::Scalar(0.03)),
            ("gamma_lv", ParameterValue::Scalar(0.072)),
        ],
    ));
    assert!(v.is_finite(), "contact angle should be finite, got {v}");
}
