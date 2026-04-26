use sciforge::hub::prelude::*;

fn run_phys(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Physics, name);
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
fn hooke_stress() {
    let v = scalar(run_phys(
        "hooke_stress",
        vec![
            ("e", ParameterValue::Scalar(200e9)),
            ("strain", ParameterValue::Scalar(0.001)),
        ],
    ));
    assert!((v - 200e6).abs() < 1.0, "sigma=E*eps=200MPa, got {v}");
}

#[test]
fn von_mises() {
    let v = scalar(run_phys(
        "von_mises",
        vec![
            ("s1", ParameterValue::Scalar(100.0)),
            ("s2", ParameterValue::Scalar(0.0)),
            ("s3", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 100.0).abs() < 1e-6, "uniaxial => VM=sigma, got {v}");
}

#[test]
fn thermal_stress() {
    let v = scalar(run_phys(
        "thermal_stress",
        vec![
            ("e", ParameterValue::Scalar(200e9)),
            ("alpha", ParameterValue::Scalar(12e-6)),
            ("delta_t", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!(v.abs() < 1e-6, "dT=0 => no thermal stress, got {v}");
}
