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
fn fluvial_erosion_rate() {
    let v = scalar(run_geo(
        "fluvial_erosion_rate",
        vec![
            ("k", ParameterValue::Scalar(1e-4)),
            ("p", ParameterValue::Scalar(1.0)),
            ("alpha", ParameterValue::Scalar(0.5)),
            ("vc", ParameterValue::Scalar(0.5)),
        ],
    ));
    assert!(v > 0.0, "erosion rate should be positive, got {v}");
}

#[test]
fn chemical_weathering_rate() {
    let v = scalar(run_geo(
        "chemical_weathering_rate",
        vec![
            ("a", ParameterValue::Scalar(1e6)),
            ("ea", ParameterValue::Scalar(50000.0)),
            ("t", ParameterValue::Scalar(288.0)),
            ("p", ParameterValue::Scalar(1000.0)),
        ],
    ));
    assert!(v > 0.0, "weathering rate should be positive, got {v}");
}

#[test]
fn frost_weathering_rate() {
    let v = scalar(run_geo(
        "frost_weathering_rate",
        vec![
            ("n_ft", ParameterValue::Scalar(50.0)),
            ("phi", ParameterValue::Scalar(0.3)),
        ],
    ));
    assert!(v > 0.0, "frost weathering should be positive, got {v}");
}

#[test]
fn wind_erosion_threshold() {
    let v = scalar(run_geo(
        "wind_erosion_threshold",
        vec![
            ("rho_p", ParameterValue::Scalar(2650.0)),
            ("rho_a", ParameterValue::Scalar(1.2)),
            ("g", ParameterValue::Scalar(9.81)),
            ("d", ParameterValue::Scalar(1e-4)),
        ],
    ));
    assert!(v > 0.0, "threshold should be positive, got {v}");
}

#[test]
fn volcanic_explosivity_index() {
    let v = scalar(run_geo(
        "volcanic_explosivity_index",
        vec![("volume_km3", ParameterValue::Scalar(0.01))],
    ));
    assert!((0.0..=8.0).contains(&v), "VEI 0-8, got {v}");
}
