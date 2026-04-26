use sciforge::hub::prelude::*;

fn run_chem(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Chemistry, name);
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
fn beer_lambert() {
    let v = scalar(run_chem(
        "beer_lambert",
        vec![
            ("epsilon", ParameterValue::Scalar(100.0)),
            ("path_length", ParameterValue::Scalar(1.0)),
            ("concentration", ParameterValue::Scalar(0.01)),
        ],
    ));
    assert!(
        (v - 1.0).abs() < 1e-10,
        "A = εlc = 100·1·0.01 = 1.0, got {v}"
    );
}

#[test]
fn absorbance_transmittance_roundtrip() {
    let t = scalar(run_chem(
        "absorbance_to_transmittance",
        vec![("absorbance", ParameterValue::Scalar(2.0))],
    ));
    assert!((t - 0.01).abs() < 1e-10, "A=2 → T=0.01, got {t}");
    let a = scalar(run_chem(
        "transmittance_to_absorbance",
        vec![("transmittance", ParameterValue::Scalar(t))],
    ));
    assert!((a - 2.0).abs() < 1e-10, "roundtrip: got {a}");
}

#[test]
fn dilution() {
    let v = scalar(run_chem(
        "dilution",
        vec![
            ("c1", ParameterValue::Scalar(1.0)),
            ("v1", ParameterValue::Scalar(0.01)),
            ("v2", ParameterValue::Scalar(0.1)),
        ],
    ));
    assert!((v - 0.1).abs() < 1e-10, "C1V1=C2V2 → C2=0.1, got {v}");
}

#[test]
fn signal_to_noise() {
    let v = scalar(run_chem(
        "signal_to_noise",
        vec![
            ("signal", ParameterValue::Scalar(100.0)),
            ("noise", ParameterValue::Scalar(5.0)),
        ],
    ));
    assert!((v - 20.0).abs() < 1e-10);
}

#[test]
fn percent_recovery() {
    let v = scalar(run_chem(
        "percent_recovery",
        vec![
            ("measured", ParameterValue::Scalar(95.0)),
            ("expected", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!((v - 95.0).abs() < 1e-10);
}

#[test]
fn retention_factor() {
    let v = scalar(run_chem(
        "retention_factor_rf",
        vec![
            ("distance_solute", ParameterValue::Scalar(3.5)),
            ("distance_solvent", ParameterValue::Scalar(7.0)),
        ],
    ));
    assert!((v - 0.5).abs() < 1e-10, "Rf = 3.5/7 = 0.5");
}
