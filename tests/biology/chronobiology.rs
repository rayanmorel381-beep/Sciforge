use sciforge_hub::prelude::*;

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
fn zeitgeber_strength() {
    let v = scalar(run_bio(
        "zeitgeber_strength",
        vec![
            ("light_intensity", ParameterValue::Scalar(1000.0)),
            ("threshold", ParameterValue::Scalar(500.0)),
            ("saturation", ParameterValue::Scalar(2000.0)),
        ],
    ));
    assert!(
        v >= 0.0 && v.is_finite(),
        "strength should be finite, got {v}"
    );
}

#[test]
fn jet_lag_recovery() {
    let v = scalar(run_bio(
        "jet_lag_recovery",
        vec![
            ("timezone_shift", ParameterValue::Scalar(6.0)),
            ("adaptation_rate", ParameterValue::Scalar(1.0)),
            ("days", ParameterValue::Scalar(7.0)),
        ],
    ));
    assert!(v.is_finite(), "recovery should be finite, got {v}");
}

#[test]
fn photoperiod() {
    let v = scalar(run_bio(
        "photoperiod",
        vec![
            ("latitude_rad", ParameterValue::Scalar(0.0)),
            ("declination_rad", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 12.0).abs() < 0.5, "equinox at equator ~12h, got {v}");
}

#[test]
fn circadian_amplitude_damping() {
    let v = scalar(run_bio(
        "circadian_amplitude_damping",
        vec![
            ("initial_amplitude", ParameterValue::Scalar(1.0)),
            ("damping_rate", ParameterValue::Scalar(0.1)),
            ("t", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!(
        (v - 1.0).abs() < 1e-8,
        "at t=0, amplitude unchanged, got {v}"
    );
}
