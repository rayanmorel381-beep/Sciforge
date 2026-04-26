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
fn basic_reproduction_number_viral() {
    let v = scalar(run_bio(
        "basic_reproduction_number_viral",
        vec![
            ("beta", ParameterValue::Scalar(1e-7)),
            ("p", ParameterValue::Scalar(100.0)),
            ("x0", ParameterValue::Scalar(1e6)),
            ("delta", ParameterValue::Scalar(0.5)),
            ("c", ParameterValue::Scalar(3.0)),
        ],
    ));
    assert!(v > 0.0 && v.is_finite(), "R0 should be positive, got {v}");
}

#[test]
fn viral_load_decay() {
    let v = scalar(run_bio(
        "viral_load_decay",
        vec![
            ("v0", ParameterValue::Scalar(1e6)),
            ("clearance", ParameterValue::Scalar(0.5)),
            ("t", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 1e6).abs() < 1.0, "at t=0, V=V0, got {v}");
}

#[test]
fn herd_immunity_threshold() {
    let v = scalar(run_bio(
        "herd_immunity_threshold",
        vec![("r0", ParameterValue::Scalar(3.0))],
    ));
    let expected = 1.0 - 1.0 / 3.0;
    assert!(
        (v - expected).abs() < 1e-8,
        "HIT=1-1/R0={expected}, got {v}"
    );
}

#[test]
fn burst_size() {
    let v = scalar(run_bio(
        "burst_size",
        vec![
            ("p", ParameterValue::Scalar(200.0)),
            ("delta", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!((v - 20.0).abs() < 1e-8, "burst=p/delta=20, got {v}");
}
