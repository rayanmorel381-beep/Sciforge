use sciforge::hub::prelude::*;

fn run_geo(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Geology, name);
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
fn p_wave_velocity() {
    let v = scalar(run_geo(
        "p_wave_velocity",
        vec![
            ("k", ParameterValue::Scalar(130e9)),
            ("g", ParameterValue::Scalar(68e9)),
            ("rho", ParameterValue::Scalar(3300.0)),
        ],
    ));
    assert!(
        v > 7000.0 && v < 9000.0,
        "P-wave in mantle ~8 km/s, got {v} m/s"
    );
}

#[test]
fn s_wave_velocity() {
    let v = scalar(run_geo(
        "s_wave_velocity",
        vec![
            ("g", ParameterValue::Scalar(68e9)),
            ("rho", ParameterValue::Scalar(3300.0)),
        ],
    ));
    assert!(
        v > 3000.0 && v < 6000.0,
        "S-wave in mantle ~4.5 km/s, got {v} m/s"
    );
}

#[test]
fn p_wave_faster_than_s_wave() {
    let vp = scalar(run_geo(
        "p_wave_velocity",
        vec![
            ("k", ParameterValue::Scalar(130e9)),
            ("g", ParameterValue::Scalar(68e9)),
            ("rho", ParameterValue::Scalar(3300.0)),
        ],
    ));
    let vs = scalar(run_geo(
        "s_wave_velocity",
        vec![
            ("g", ParameterValue::Scalar(68e9)),
            ("rho", ParameterValue::Scalar(3300.0)),
        ],
    ));
    assert!(vp > vs, "P-waves must be faster than S-waves");
}

#[test]
fn richter_magnitude() {
    let v = scalar(run_geo(
        "richter_magnitude",
        vec![
            ("amplitude", ParameterValue::Scalar(10.0)),
            ("distance_km", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!(v.is_finite(), "Richter should return finite value");
}

#[test]
fn seismic_energy() {
    let e5 = scalar(run_geo(
        "seismic_energy",
        vec![("magnitude", ParameterValue::Scalar(5.0))],
    ));
    let e6 = scalar(run_geo(
        "seismic_energy",
        vec![("magnitude", ParameterValue::Scalar(6.0))],
    ));
    assert!(
        e6 / e5 > 25.0,
        "+1 magnitude ≈ 31.6× energy, got ratio {}",
        e6 / e5
    );
}

#[test]
fn gutenberg_richter() {
    let v = scalar(run_geo(
        "gutenberg_richter",
        vec![
            ("a", ParameterValue::Scalar(5.0)),
            ("b", ParameterValue::Scalar(1.0)),
            ("magnitude", ParameterValue::Scalar(3.0)),
        ],
    ));
    let expected = 10.0_f64.powf(5.0 - 1.0 * 3.0);
    assert!(
        (v - expected).abs() / expected < 1e-6,
        "got {v}, expected {expected}"
    );
}

#[test]
fn gutenberg_richter_decreases_with_magnitude() {
    let n3 = scalar(run_geo(
        "gutenberg_richter",
        vec![
            ("a", ParameterValue::Scalar(6.0)),
            ("b", ParameterValue::Scalar(1.0)),
            ("magnitude", ParameterValue::Scalar(3.0)),
        ],
    ));
    let n5 = scalar(run_geo(
        "gutenberg_richter",
        vec![
            ("a", ParameterValue::Scalar(6.0)),
            ("b", ParameterValue::Scalar(1.0)),
            ("magnitude", ParameterValue::Scalar(5.0)),
        ],
    ));
    assert!(n5 < n3);
}

#[test]
fn richter_amplitude_ratio_adds_one_magnitude() {
    let m1 = scalar(run_geo(
        "richter_magnitude",
        vec![
            ("amplitude", ParameterValue::Scalar(10.0)),
            ("distance_km", ParameterValue::Scalar(100.0)),
        ],
    ));
    let m2 = scalar(run_geo(
        "richter_magnitude",
        vec![
            ("amplitude", ParameterValue::Scalar(100.0)),
            ("distance_km", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!((m2 - m1 - 1.0).abs() < 1e-10);
}
