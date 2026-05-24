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
fn mg_number() {
    let v = scalar(run_geo(
        "mg_number",
        vec![
            ("mgo", ParameterValue::Scalar(10.0)),
            ("feo", ParameterValue::Scalar(8.0)),
        ],
    ));
    assert!(v > 40.0 && v < 70.0, "Mg# for typical basalt, got {v}");
}

#[test]
fn total_alkali_silica() {
    let v = scalar(run_geo(
        "total_alkali_silica",
        vec![
            ("na2o", ParameterValue::Scalar(2.0)),
            ("k2o", ParameterValue::Scalar(2.0)),
        ],
    ));
    assert!((v - 4.0).abs() < 0.01, "TAS = Na2O + K2O = 4, got {v}");
}

#[test]
fn differentiation_index() {
    let v = scalar(run_geo(
        "differentiation_index",
        vec![
            ("q", ParameterValue::Scalar(5.0)),
            ("or_val", ParameterValue::Scalar(15.0)),
            ("ab", ParameterValue::Scalar(35.0)),
            ("ne", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 55.0).abs() < 0.1, "DI = Q+Or+Ab+Ne = 55, got {v}");
}

#[test]
fn cipw_quartz_norm() {
    let v = scalar(run_geo(
        "cipw_quartz_norm",
        vec![
            ("sio2", ParameterValue::Scalar(60.0)),
            ("feldspars", ParameterValue::Scalar(30.0)),
            ("mafics", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!(
        v > 0.0,
        "quartz norm should be positive for silicic rock, got {v}"
    );
}

#[test]
fn crystal_settling_velocity() {
    let v = scalar(run_geo(
        "crystal_settling_velocity",
        vec![
            ("delta_rho", ParameterValue::Scalar(200.0)),
            ("g", ParameterValue::Scalar(9.81)),
            ("r", ParameterValue::Scalar(1e-3)),
            ("mu", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!(v > 0.0, "settling velocity should be positive, got {v}");
}

#[test]
fn liquidus_temperature() {
    let v = scalar(run_geo(
        "liquidus_temperature",
        vec![
            ("composition", ParameterValue::Scalar(0.5)),
            ("t_melt_a", ParameterValue::Scalar(1800.0)),
            ("t_melt_b", ParameterValue::Scalar(1400.0)),
        ],
    ));
    assert!(
        v > 1400.0 && v < 1800.0,
        "liquidus between endmembers, got {v}"
    );
}

#[test]
fn viscosity_arrhenius() {
    let v = scalar(run_geo(
        "viscosity_arrhenius",
        vec![
            ("a", ParameterValue::Scalar(1e10)),
            ("ea", ParameterValue::Scalar(200000.0)),
            ("t", ParameterValue::Scalar(1473.0)),
        ],
    ));
    assert!(v > 0.0, "viscosity should be positive, got {v}");
}
