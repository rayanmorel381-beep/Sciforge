use sciforge::hub::prelude::*;

fn run_meteo(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Meteorology, name);
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
fn coriolis_parameter() {
    let lat = 45.0_f64.to_radians();
    let v = scalar(run_meteo(
        "coriolis_parameter",
        vec![
            ("latitude", ParameterValue::Scalar(lat)),
            ("omega", ParameterValue::Scalar(7.292115e-5)),
        ],
    ));
    let omega = 7.292_115e-5;
    let expected = 2.0 * omega * lat.sin();
    assert!(
        (v - expected).abs() / expected < 1e-4,
        "f at 45° ≈ 1.03e-4, got {v}"
    );
}

#[test]
fn coriolis_zero_at_equator() {
    let v = scalar(run_meteo(
        "coriolis_parameter",
        vec![
            ("latitude", ParameterValue::Scalar(0.0)),
            ("omega", ParameterValue::Scalar(7.292115e-5)),
        ],
    ));
    assert!(v.abs() < 1e-12, "f = 0 at equator, got {v}");
}

#[test]
fn rossby_number() {
    let v = scalar(run_meteo(
        "rossby_number",
        vec![
            ("u", ParameterValue::Scalar(10.0)),
            ("l", ParameterValue::Scalar(1e6)),
            ("f", ParameterValue::Scalar(1e-4)),
        ],
    ));
    assert!((v - 0.1).abs() < 1e-6, "Ro = U/(fL) = 0.1, got {v}");
}

#[test]
fn geostrophic_wind() {
    let out = run_meteo(
        "geostrophic_wind",
        vec![
            ("dp_dx", ParameterValue::Scalar(0.001)),
            ("dp_dy", ParameterValue::Scalar(0.0)),
            ("rho", ParameterValue::Scalar(1.225)),
            ("f", ParameterValue::Scalar(1e-4)),
        ],
    );
    match out {
        RunOutput::Pair(ug, vg) => {
            assert!(ug.is_finite() && vg.is_finite());
        }
        _ => panic!("expected Pair, got {out:?}"),
    }
}

#[test]
fn ekman_depth() {
    let v = scalar(run_meteo(
        "ekman_depth",
        vec![
            ("nu", ParameterValue::Scalar(0.1)),
            ("f", ParameterValue::Scalar(1e-4)),
        ],
    ));
    assert!(v > 0.0, "Ekman depth should be positive");
}

#[test]
fn rossby_wave_speed() {
    let v = scalar(run_meteo(
        "rossby_wave_speed",
        vec![
            ("beta", ParameterValue::Scalar(1.6e-11)),
            ("k", ParameterValue::Scalar(1e-6)),
        ],
    ));
    assert!(v.is_finite(), "Rossby wave speed should be finite");
}

#[test]
fn coriolis_changes_sign_between_hemispheres() {
    let omega = 7.292115e-5;
    let f_n = scalar(run_meteo(
        "coriolis_parameter",
        vec![
            ("latitude", ParameterValue::Scalar(30.0_f64.to_radians())),
            ("omega", ParameterValue::Scalar(omega)),
        ],
    ));
    let f_s = scalar(run_meteo(
        "coriolis_parameter",
        vec![
            ("latitude", ParameterValue::Scalar((-30.0_f64).to_radians())),
            ("omega", ParameterValue::Scalar(omega)),
        ],
    ));
    assert!((f_n + f_s).abs() < 1e-12);
}

#[test]
fn rossby_number_decreases_with_scale() {
    let ro_small = scalar(run_meteo(
        "rossby_number",
        vec![
            ("u", ParameterValue::Scalar(10.0)),
            ("l", ParameterValue::Scalar(1e5)),
            ("f", ParameterValue::Scalar(1e-4)),
        ],
    ));
    let ro_large = scalar(run_meteo(
        "rossby_number",
        vec![
            ("u", ParameterValue::Scalar(10.0)),
            ("l", ParameterValue::Scalar(1e6)),
            ("f", ParameterValue::Scalar(1e-4)),
        ],
    ));
    assert!(ro_large < ro_small);
}
