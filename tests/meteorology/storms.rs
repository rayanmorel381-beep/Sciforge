use sciforge_hub::prelude::*;

fn run_meteo(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Meteorology, name);
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
fn potential_intensity() {
    let v = scalar(run_meteo(
        "potential_intensity",
        vec![
            ("ck", ParameterValue::Scalar(1.1)),
            ("cd", ParameterValue::Scalar(1.0)),
            ("eta", ParameterValue::Scalar(0.6)),
            ("delta_k", ParameterValue::Scalar(30.0)),
        ],
    ));
    assert!(v > 0.0, "potential intensity should be positive, got {v}");
}

#[test]
fn cape_positive_buoyancy() {
    let v = scalar(run_meteo(
        "cape",
        vec![
            ("t_parcel", ParameterValue::Scalar(320.0)),
            ("t_env", ParameterValue::Scalar(300.0)),
            ("g", ParameterValue::Scalar(9.81)),
            ("dz", ParameterValue::Scalar(1000.0)),
        ],
    ));
    assert!(
        v > 0.0,
        "CAPE should be positive for warmer parcel, got {v}"
    );
}

#[test]
fn cape_zero_for_equal_temps() {
    let v = scalar(run_meteo(
        "cape",
        vec![
            ("t_parcel", ParameterValue::Scalar(300.0)),
            ("t_env", ParameterValue::Scalar(300.0)),
            ("g", ParameterValue::Scalar(9.81)),
            ("dz", ParameterValue::Scalar(1000.0)),
        ],
    ));
    assert!(
        v.abs() < 1e-6,
        "CAPE should be zero when T_parcel = T_env, got {v}"
    );
}

#[test]
fn fujita_scale() {
    let v = scalar(run_meteo(
        "fujita_scale",
        vec![("v", ParameterValue::Scalar(50.0))],
    ));
    assert!((0.0..=5.0).contains(&v), "Fujita scale 0-5, got {v}");
}

#[test]
fn fujita_scale_increases_with_wind() {
    let f1 = scalar(run_meteo(
        "fujita_scale",
        vec![("v", ParameterValue::Scalar(30.0))],
    ));
    let f2 = scalar(run_meteo(
        "fujita_scale",
        vec![("v", ParameterValue::Scalar(80.0))],
    ));
    assert!(
        f2 >= f1,
        "stronger wind -> higher Fujita, got f1={f1}, f2={f2}"
    );
}
