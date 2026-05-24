use sciforge_hub::prelude::*;

fn run_astro(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Astronomy, name);
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
fn tidal_potential() {
    let v = scalar(run_astro(
        "tidal_potential",
        vec![
            ("m", ParameterValue::Scalar(7.342e22)),
            ("r", ParameterValue::Scalar(6.371e6)),
            ("d", ParameterValue::Scalar(3.844e8)),
            ("k2", ParameterValue::Scalar(0.3)),
            ("theta", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!(v.is_finite(), "tidal potential should be finite, got {v}");
}

#[test]
fn spring_tide_amplitude() {
    let v = scalar(run_astro(
        "spring_tide_amplitude",
        vec![
            ("h_moon", ParameterValue::Scalar(0.54)),
            ("h_sun", ParameterValue::Scalar(0.25)),
        ],
    ));
    assert!(v > 0.0, "spring tide amplitude should be positive, got {v}");
}

#[test]
fn tidal_locking_timescale() {
    let omega = 2.0 * std::f64::consts::PI / (27.3 * 86400.0);
    let v = scalar(run_astro(
        "tidal_locking_timescale",
        vec![
            ("omega", ParameterValue::Scalar(omega)),
            ("a", ParameterValue::Scalar(3.844e8)),
            ("mu", ParameterValue::Scalar(6.5e10)),
            ("q", ParameterValue::Scalar(100.0)),
            ("m", ParameterValue::Scalar(5.972e24)),
            ("r", ParameterValue::Scalar(1.737e6)),
        ],
    ));
    assert!(v > 0.0, "locking timescale should be positive, got {v}");
}
