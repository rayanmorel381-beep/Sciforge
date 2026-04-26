use sciforge::hub::prelude::*;

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
fn nernst_potential() {
    let v = scalar(run_chem(
        "nernst_potential",
        vec![
            ("e0", ParameterValue::Scalar(0.0)),
            ("n", ParameterValue::Scalar(2.0)),
            ("q", ParameterValue::Scalar(1.0)),
            ("t", ParameterValue::Scalar(298.15)),
        ],
    ));
    assert!(v.is_finite(), "Nernst should be finite at Q=1 → E = E0");
}

#[test]
fn half_life_first_order() {
    let v = scalar(run_chem(
        "half_life_first_order",
        vec![("k", ParameterValue::Scalar(0.1))],
    ));
    let expected = std::f64::consts::LN_2 / 0.1;
    assert!((v - expected).abs() < 1e-6, "t½ = ln2/k, got {v}");
}

#[test]
fn rate_constant_arrhenius() {
    let v = scalar(run_chem(
        "rate_constant_arrhenius",
        vec![
            ("a", ParameterValue::Scalar(1e13)),
            ("ea", ParameterValue::Scalar(75000.0)),
            ("t", ParameterValue::Scalar(298.15)),
        ],
    ));
    assert!(v > 0.0 && v < 1e13, "k should be between 0 and A");
}

#[test]
fn arrhenius_temperature_effect() {
    let k1 = scalar(run_chem(
        "rate_constant_arrhenius",
        vec![
            ("a", ParameterValue::Scalar(1e13)),
            ("ea", ParameterValue::Scalar(75000.0)),
            ("t", ParameterValue::Scalar(300.0)),
        ],
    ));
    let k2 = scalar(run_chem(
        "rate_constant_arrhenius",
        vec![
            ("a", ParameterValue::Scalar(1e13)),
            ("ea", ParameterValue::Scalar(75000.0)),
            ("t", ParameterValue::Scalar(350.0)),
        ],
    ));
    assert!(k2 > k1, "rate constant should increase with temperature");
}

#[test]
fn gibbs_keq_roundtrip() {
    let g = scalar(run_chem(
        "gibbs_from_keq",
        vec![
            ("keq", ParameterValue::Scalar(100.0)),
            ("t", ParameterValue::Scalar(298.15)),
        ],
    ));
    let keq = scalar(run_chem(
        "equilibrium_constant_from_gibbs",
        vec![
            ("delta_g", ParameterValue::Scalar(g)),
            ("t", ParameterValue::Scalar(298.15)),
        ],
    ));
    assert!(
        (keq - 100.0).abs() / 100.0 < 1e-6,
        "Keq roundtrip: expected 100, got {keq}"
    );
}

#[test]
fn cell_potential() {
    let v = scalar(run_chem(
        "cell_potential",
        vec![
            ("e_cathode", ParameterValue::Scalar(0.34)),
            ("e_anode", ParameterValue::Scalar(-0.76)),
        ],
    ));
    assert!(
        (v - 1.10).abs() < 1e-10,
        "E° = 0.34 - (-0.76) = 1.10 V, got {v}"
    );
}

#[test]
fn dispatch_unknown_function_returns_error() {
    let exp = Experiment::new(DomainType::Chemistry, "unknown_chemistry_function");
    let result = ExperimentRunner::new().run(&exp);
    assert!(result.is_err());
}

#[test]
fn dispatch_missing_parameter_returns_error() {
    let exp = Experiment::new(DomainType::Chemistry, "half_life_first_order");
    let result = ExperimentRunner::new().run(&exp);
    assert!(result.is_err());
}
