use sciforge_hub::prelude::*;

fn run_geo(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Geology, name);
    for (k, v) in params {
        exp = exp.param(k, v);
    }
    ExperimentRunner::new()
        .run(&exp)
        .unwrap_or_else(|_| panic!("dispatch \'{name}\' failed"))
}

fn scalar(o: RunOutput) -> f64 {
    match o {
        RunOutput::Scalar(v) => v,
        _ => panic!("expected Scalar, got {o:?}"),
    }
}

#[test]
fn glen_strain_rate() {
    let v = scalar(run_geo(
        "glen_strain_rate",
        vec![
            ("a", ParameterValue::Scalar(1e-25)),
            ("tau", ParameterValue::Scalar(1e5)),
            ("n", ParameterValue::Scalar(3.0)),
        ],
    ));
    assert!(v > 0.0, "strain rate should be positive, got {v}");
}

#[test]
fn shallow_ice_velocity() {
    let v = scalar(run_geo(
        "shallow_ice_velocity",
        vec![
            ("a", ParameterValue::Scalar(1e-25)),
            ("n", ParameterValue::Scalar(3.0)),
            ("rho", ParameterValue::Scalar(917.0)),
            ("g", ParameterValue::Scalar(9.81)),
            ("alpha", ParameterValue::Scalar(0.1)),
            ("h", ParameterValue::Scalar(1000.0)),
        ],
    ));
    assert!(v > 0.0, "velocity should be positive, got {v}");
}

#[test]
fn ice_viscosity() {
    let v = scalar(run_geo(
        "ice_viscosity",
        vec![
            ("a", ParameterValue::Scalar(1e-25)),
            ("tau", ParameterValue::Scalar(1e5)),
            ("n", ParameterValue::Scalar(3.0)),
        ],
    ));
    assert!(v > 0.0, "viscosity should be positive, got {v}");
}

#[test]
fn glacial_bed_erosion() {
    let v = scalar(run_geo(
        "glacial_bed_erosion",
        vec![
            ("kg", ParameterValue::Scalar(1e-8)),
            ("vb", ParameterValue::Scalar(100.0)),
            ("l", ParameterValue::Scalar(2.0)),
        ],
    ));
    assert!(v > 0.0, "erosion should be positive, got {v}");
}

#[test]
fn glen_strain_increases_with_stress() {
    let s1 = scalar(run_geo(
        "glen_strain_rate",
        vec![
            ("a", ParameterValue::Scalar(1e-25)),
            ("tau", ParameterValue::Scalar(5e4)),
            ("n", ParameterValue::Scalar(3.0)),
        ],
    ));
    let s2 = scalar(run_geo(
        "glen_strain_rate",
        vec![
            ("a", ParameterValue::Scalar(1e-25)),
            ("tau", ParameterValue::Scalar(1e5)),
            ("n", ParameterValue::Scalar(3.0)),
        ],
    ));
    assert!(s2 > s1, "higher stress -> higher strain rate");
}
