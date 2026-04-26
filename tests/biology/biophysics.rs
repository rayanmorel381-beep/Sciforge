use sciforge::hub::prelude::*;

fn run_bio(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Biology, name);
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
fn membrane_tension() {
    let v = scalar(run_bio(
        "membrane_tension",
        vec![
            ("area_strain", ParameterValue::Scalar(0.02)),
            ("stretch_modulus", ParameterValue::Scalar(0.2)),
        ],
    ));
    assert!((v - 0.004).abs() < 1e-8, "T=strain*K=0.004, got {v}");
}

#[test]
fn lennard_jones() {
    let v = scalar(run_bio(
        "lennard_jones",
        vec![
            ("r", ParameterValue::Scalar(1.0)),
            ("epsilon", ParameterValue::Scalar(1.0)),
            ("sigma", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((v - 0.0).abs() < 1e-8, "LJ at r=sigma is 0, got {v}");
}

#[test]
fn harmonic_bond() {
    let v = scalar(run_bio(
        "harmonic_bond",
        vec![
            ("r", ParameterValue::Scalar(1.5)),
            ("k", ParameterValue::Scalar(100.0)),
            ("r0", ParameterValue::Scalar(1.0)),
        ],
    ));
    let expected = 0.5 * 100.0 * 0.25;
    assert!((v - expected).abs() < 1e-8, "0.5*k*(r-r0)^2=12.5, got {v}");
}

#[test]
fn fret_efficiency() {
    let v = scalar(run_bio(
        "fret_efficiency",
        vec![
            ("r", ParameterValue::Scalar(5.0)),
            ("r0", ParameterValue::Scalar(5.0)),
        ],
    ));
    assert!((v - 0.5).abs() < 1e-8, "E=0.5 when r=R0, got {v}");
}
