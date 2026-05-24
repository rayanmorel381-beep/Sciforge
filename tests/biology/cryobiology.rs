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
fn freezing_point_depression() {
    let v = scalar(run_bio(
        "freezing_point_depression",
        vec![
            ("concentration", ParameterValue::Scalar(1.0)),
            ("kf", ParameterValue::Scalar(1.86)),
            ("dissociation_factor", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((v - 1.86).abs() < 1e-6, "dT=Kf*m*i=1.86, got {v}");
}

#[test]
fn critical_cooling_rate() {
    let v = scalar(run_bio(
        "critical_cooling_rate",
        vec![
            ("cpa_concentration", ParameterValue::Scalar(0.5)),
            ("base_rate", ParameterValue::Scalar(1e6)),
            ("sensitivity", ParameterValue::Scalar(5.0)),
        ],
    ));
    assert!(v > 0.0 && v.is_finite(), "CCR should be positive, got {v}");
}

#[test]
fn vitrification_probability() {
    let v = scalar(run_bio(
        "vitrification_probability",
        vec![
            ("cooling_rate", ParameterValue::Scalar(1e6)),
            ("critical_rate", ParameterValue::Scalar(1e3)),
        ],
    ));
    assert!(
        v > 0.0,
        "high cooling rate should give high vitrification prob, got {v}"
    );
}

#[test]
fn storage_decay_arrhenius() {
    let v = scalar(run_bio(
        "storage_decay_arrhenius",
        vec![
            ("a", ParameterValue::Scalar(1e10)),
            ("ea", ParameterValue::Scalar(50000.0)),
            ("temperature_k", ParameterValue::Scalar(310.0)),
        ],
    ));
    assert!(
        v > 0.0 && v.is_finite(),
        "decay rate should be positive, got {v}"
    );
}
