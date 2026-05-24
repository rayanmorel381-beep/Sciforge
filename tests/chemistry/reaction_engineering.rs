use sciforge_hub::prelude::*;

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
fn cstr_volume() {
    let v = scalar(run_chem(
        "cstr_volume",
        vec![
            ("f_a0", ParameterValue::Scalar(10.0)),
            ("x", ParameterValue::Scalar(0.9)),
            ("r_a", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(v > 0.0, "CSTR volume should be positive, got {v}");
}

#[test]
fn pfr_conversion_first_order() {
    let v = scalar(run_chem(
        "pfr_conversion_first_order",
        vec![
            ("k", ParameterValue::Scalar(1.0)),
            ("tau", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!(v.abs() < 1e-8, "tau=0 => conversion=0, got {v}");
}

#[test]
fn damkohler_number() {
    let v = scalar(run_chem(
        "damkohler_number",
        vec![
            ("k", ParameterValue::Scalar(2.0)),
            ("tau", ParameterValue::Scalar(5.0)),
            ("c0", ParameterValue::Scalar(1.0)),
            ("order", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((v - 10.0).abs() < 1e-8, "Da=k*tau=10, got {v}");
}
