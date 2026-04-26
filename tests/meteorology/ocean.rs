use sciforge::hub::prelude::*;

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
fn seawater_density() {
    let v = scalar(run_meteo(
        "seawater_density",
        vec![
            ("t", ParameterValue::Scalar(15.0)),
            ("s", ParameterValue::Scalar(35.0)),
        ],
    ));
    assert!(
        v > 1020.0 && v < 1030.0,
        "seawater density ~1025 kg/m³, got {v}"
    );
}

#[test]
fn seawater_density_increases_with_salinity() {
    let d1 = scalar(run_meteo(
        "seawater_density",
        vec![
            ("t", ParameterValue::Scalar(15.0)),
            ("s", ParameterValue::Scalar(30.0)),
        ],
    ));
    let d2 = scalar(run_meteo(
        "seawater_density",
        vec![
            ("t", ParameterValue::Scalar(15.0)),
            ("s", ParameterValue::Scalar(35.0)),
        ],
    ));
    assert!(d2 > d1, "density should increase with salinity");
}

#[test]
fn sound_speed() {
    let v = scalar(run_meteo(
        "sound_speed",
        vec![
            ("t", ParameterValue::Scalar(10.0)),
            ("s", ParameterValue::Scalar(35.0)),
            ("d", ParameterValue::Scalar(1000.0)),
        ],
    ));
    assert!(
        v > 1450.0 && v < 1550.0,
        "sound speed in ocean ~1500 m/s, got {v}"
    );
}

#[test]
fn sea_level_rise_thermal() {
    let v = scalar(run_meteo(
        "sea_level_rise_thermal",
        vec![
            ("alpha", ParameterValue::Scalar(2e-4)),
            ("delta_t", ParameterValue::Scalar(1.0)),
            ("d", ParameterValue::Scalar(4000.0)),
        ],
    ));
    assert!(
        v > 0.5 && v < 1.5,
        "thermal expansion ~0.8 m for 4km ocean, got {v}"
    );
}

#[test]
fn henry_solubility() {
    let v = scalar(run_meteo(
        "henry_solubility",
        vec![
            ("kh", ParameterValue::Scalar(3.4e-2)),
            ("t", ParameterValue::Scalar(298.15)),
            ("delta_h", ParameterValue::Scalar(2400.0)),
        ],
    ));
    assert!(v > 0.0, "Henry solubility should be positive, got {v}");
}
