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
fn tumor_growth_exponential() {
    let v = scalar(run_bio(
        "tumor_growth_exponential",
        vec![
            ("n0", ParameterValue::Scalar(100.0)),
            ("rate", ParameterValue::Scalar(0.0)),
            ("t", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!((v - 100.0).abs() < 1e-6, "no growth => N=N0, got {v}");
}

#[test]
fn cancer_linear_quadratic_survival() {
    let v = scalar(run_bio(
        "cancer_linear_quadratic_survival",
        vec![
            ("dose", ParameterValue::Scalar(0.0)),
            ("alpha", ParameterValue::Scalar(0.3)),
            ("beta", ParameterValue::Scalar(0.03)),
        ],
    ));
    assert!((v - 1.0).abs() < 1e-8, "dose=0 => survival=1, got {v}");
}

#[test]
fn tumor_doubling_time() {
    let v = scalar(run_bio(
        "tumor_doubling_time",
        vec![("growth_rate", ParameterValue::Scalar(0.1))],
    ));
    let expected = (2.0_f64).ln() / 0.1;
    assert!((v - expected).abs() < 1e-6, "Td=ln2/r, got {v}");
}

#[test]
fn cell_kill_log() {
    let v = scalar(run_bio(
        "cell_kill_log",
        vec![
            ("initial", ParameterValue::Scalar(1e6)),
            ("surviving_fraction", ParameterValue::Scalar(0.01)),
            ("cycles", ParameterValue::Integer(1)),
        ],
    ));
    assert!(v.is_finite(), "cell kill log should be finite, got {v}");
}
