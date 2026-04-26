use sciforge::hub::prelude::*;

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
fn radiometric_age() {
    let v = scalar(run_bio(
        "radiometric_age",
        vec![
            ("parent", ParameterValue::Scalar(50.0)),
            ("daughter", ParameterValue::Scalar(50.0)),
            ("decay_constant", ParameterValue::Scalar(1e-9)),
        ],
    ));
    assert!(v > 0.0, "age should be positive, got {v}");
}

#[test]
fn extinction_rate() {
    let v = scalar(run_bio(
        "extinction_rate",
        vec![
            ("extinctions", ParameterValue::Scalar(20.0)),
            ("taxa_at_start", ParameterValue::Scalar(100.0)),
            ("interval_myr", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!(v > 0.0, "extinction rate should be positive, got {v}");
}

#[test]
fn net_diversification_rate() {
    let v = scalar(run_bio(
        "net_diversification_rate",
        vec![
            ("origination", ParameterValue::Scalar(0.2)),
            ("extinction", ParameterValue::Scalar(0.1)),
        ],
    ));
    assert!((v - 0.1).abs() < 1e-8, "net=0.2-0.1=0.1, got {v}");
}

#[test]
fn evolutionary_rate_darwin() {
    let v = scalar(run_bio(
        "evolutionary_rate_darwin",
        vec![
            ("size_initial", ParameterValue::Scalar(1.0)),
            ("size_final", ParameterValue::Scalar(std::f64::consts::E)),
            ("time_myr", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(v > 0.0, "darwin rate positive for size increase, got {v}");
}
