use sciforge::hub::prelude::*;

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
fn manning_velocity() {
    let v = scalar(run_geo(
        "manning_velocity",
        vec![
            ("n", ParameterValue::Scalar(0.03)),
            ("rh", ParameterValue::Scalar(1.0)),
            ("s", ParameterValue::Scalar(0.001)),
        ],
    ));
    assert!(v > 0.0, "flow velocity should be positive, got {v}");
}

#[test]
fn chezy_velocity() {
    let v = scalar(run_geo(
        "chezy_velocity",
        vec![
            ("c", ParameterValue::Scalar(40.0)),
            ("rh", ParameterValue::Scalar(1.0)),
            ("s", ParameterValue::Scalar(0.001)),
        ],
    ));
    assert!(v > 0.0, "Chezy velocity should be positive, got {v}");
}

#[test]
fn froude_number() {
    let v = scalar(run_geo(
        "froude_number",
        vec![
            ("v", ParameterValue::Scalar(1.0)),
            ("g", ParameterValue::Scalar(9.81)),
            ("d", ParameterValue::Scalar(1.0)),
        ],
    ));
    let expected = 1.0 / (9.81_f64).sqrt();
    assert!((v - expected).abs() < 0.01, "Fr = v/√(gd), got {v}");
}

#[test]
fn reynolds_number() {
    let v = scalar(run_geo(
        "reynolds_number",
        vec![
            ("v", ParameterValue::Scalar(1.0)),
            ("d", ParameterValue::Scalar(1.0)),
            ("nu", ParameterValue::Scalar(1e-6)),
        ],
    ));
    assert!((v - 1e6).abs() / 1e6 < 0.01, "Re = v*d/nu = 1e6, got {v}");
}

#[test]
fn stream_power() {
    let v = scalar(run_geo(
        "stream_power",
        vec![
            ("rho", ParameterValue::Scalar(1000.0)),
            ("g", ParameterValue::Scalar(9.81)),
            ("q", ParameterValue::Scalar(100.0)),
            ("s", ParameterValue::Scalar(0.01)),
        ],
    ));
    assert!(v > 0.0, "stream power should be positive, got {v}");
}

#[test]
fn hjulstrom_erosion_threshold() {
    let v = scalar(run_geo(
        "hjulstrom_erosion_threshold",
        vec![("d_grain", ParameterValue::Scalar(1e-4))],
    ));
    assert!(v > 0.0, "threshold velocity should be positive, got {v}");
}
