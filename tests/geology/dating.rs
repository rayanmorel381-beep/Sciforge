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
fn radioactive_decay() {
    let v = scalar(run_geo(
        "radioactive_decay",
        vec![
            ("n0", ParameterValue::Scalar(1000.0)),
            ("lambda", ParameterValue::Scalar(0.1)),
            ("t", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 1000.0).abs() < 1e-8, "N(0) = N0 = 1000, got {v}");
}

#[test]
fn radioactive_decay_halflife() {
    let hl = scalar(run_geo(
        "half_life",
        vec![("lambda", ParameterValue::Scalar(0.1))],
    ));
    let n = scalar(run_geo(
        "radioactive_decay",
        vec![
            ("n0", ParameterValue::Scalar(1000.0)),
            ("lambda", ParameterValue::Scalar(0.1)),
            ("t", ParameterValue::Scalar(hl)),
        ],
    ));
    assert!((n - 500.0).abs() < 1e-4, "N(t½) = N0/2, got {n}");
}

#[test]
fn decay_constant() {
    let v = scalar(run_geo(
        "decay_constant",
        vec![("half_life", ParameterValue::Scalar(5730.0))],
    ));
    let expected = (2.0_f64).ln() / 5730.0;
    assert!((v - expected).abs() / expected < 1e-6, "λ = ln2/t½");
}

#[test]
fn carbon14_age() {
    let v = scalar(run_geo(
        "carbon14_age",
        vec![("ratio", ParameterValue::Scalar(0.5))],
    ));
    assert!((v - 5730.0).abs() < 100.0, "half ratio → ~5730 yr, got {v}");
}

#[test]
fn age_from_ratio() {
    let v = scalar(run_geo(
        "age_from_ratio",
        vec![
            ("ratio_daughter_parent", ParameterValue::Scalar(1.0)),
            ("lambda", ParameterValue::Scalar(0.1)),
        ],
    ));
    let expected = (2.0_f64).ln() / 0.1;
    assert!((v - expected).abs() / expected < 1e-4);
}

#[test]
fn radioactive_decay_large_time_stability() {
    let v = scalar(run_geo(
        "radioactive_decay",
        vec![
            ("n0", ParameterValue::Scalar(1.0e12)),
            ("lambda", ParameterValue::Scalar(1.0e-6)),
            ("t", ParameterValue::Scalar(1.0e6)),
        ],
    ));
    assert!(v.is_finite());
    assert!(v >= 0.0);
}

#[test]
fn half_life_positive_for_positive_lambda() {
    let hl = scalar(run_geo(
        "half_life",
        vec![("lambda", ParameterValue::Scalar(0.23))],
    ));
    assert!(hl > 0.0);
}
