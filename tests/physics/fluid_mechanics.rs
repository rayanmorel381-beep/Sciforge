use sciforge_hub::prelude::*;

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
fn reynolds_number() {
    let v = scalar(run_phys(
        "reynolds_number",
        vec![
            ("rho", ParameterValue::Scalar(1000.0)),
            ("v", ParameterValue::Scalar(1.0)),
            ("l", ParameterValue::Scalar(0.01)),
            ("mu", ParameterValue::Scalar(1e-3)),
        ],
    ));
    assert!((v - 10000.0).abs() < 1e-3, "Re=rho*v*l/mu=10000, got {v}");
}

#[test]
fn bernoulli_pressure() {
    let v = scalar(run_phys(
        "bernoulli_pressure",
        vec![
            ("p1", ParameterValue::Scalar(101325.0)),
            ("v1", ParameterValue::Scalar(0.0)),
            ("v2", ParameterValue::Scalar(0.0)),
            ("rho", ParameterValue::Scalar(1000.0)),
            ("h1", ParameterValue::Scalar(0.0)),
            ("h2", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!(
        (v - 101325.0).abs() < 1.0,
        "same conditions => same P, got {v}"
    );
}

#[test]
fn drag_force() {
    let v = scalar(run_phys(
        "drag_force",
        vec![
            ("cd", ParameterValue::Scalar(0.47)),
            ("rho", ParameterValue::Scalar(1.225)),
            ("v", ParameterValue::Scalar(10.0)),
            ("a", ParameterValue::Scalar(0.01)),
        ],
    ));
    assert!(v > 0.0, "drag force should be positive, got {v}");
}
