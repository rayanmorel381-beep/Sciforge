use sciforge_hub::prelude::*;

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
fn poincare_disk_distance() {
    let v = scalar(run_maths(
        "poincare_disk_distance",
        vec![
            ("p1", ParameterValue::Vector(vec![0.0, 0.0])),
            ("p2", ParameterValue::Vector(vec![0.0, 0.0])),
        ],
    ));
    assert!(v.abs() < 1e-8, "same point => distance=0, got {v}");
}

#[test]
fn spherical_distance() {
    let v = scalar(run_maths(
        "spherical_distance",
        vec![
            ("theta1", ParameterValue::Scalar(0.0)),
            ("phi1", ParameterValue::Scalar(0.0)),
            ("theta2", ParameterValue::Scalar(0.0)),
            ("phi2", ParameterValue::Scalar(0.0)),
            ("r", ParameterValue::Scalar(6371.0)),
        ],
    ));
    assert!(v.abs() < 1e-6, "same point => distance=0, got {v}");
}

#[test]
fn schwarzschild_radius() {
    let v = scalar(run_maths(
        "schwarzschild_radius",
        vec![("mass_kg", ParameterValue::Scalar(1.989e30))],
    ));
    assert!((v - 2953.0).abs() < 10.0, "Rs(Sun)≈2953 m, got {v}");
}

#[test]
fn hawking_temperature() {
    let v = scalar(run_maths(
        "hawking_temperature",
        vec![("mass_kg", ParameterValue::Scalar(1.989e30))],
    ));
    assert!(v > 0.0, "Hawking T should be positive, got {v}");
}
