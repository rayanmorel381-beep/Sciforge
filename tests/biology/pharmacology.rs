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
fn half_life() {
    let v = scalar(run_bio(
        "half_life",
        vec![("ke", ParameterValue::Scalar(0.1))],
    ));
    let expected = (2.0_f64).ln() / 0.1;
    assert!((v - expected).abs() < 1e-6, "t1/2=ln2/ke, got {v}");
}

#[test]
fn bioavailability() {
    let v = scalar(run_bio(
        "bioavailability",
        vec![
            ("auc_oral", ParameterValue::Scalar(50.0)),
            ("dose_oral", ParameterValue::Scalar(100.0)),
            ("auc_iv", ParameterValue::Scalar(100.0)),
            ("dose_iv", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!(
        (v - 0.5).abs() < 1e-8,
        "F=(AUCoral/Doral)/(AUCiv/Div)=0.5, got {v}"
    );
}

#[test]
fn therapeutic_index() {
    let v = scalar(run_bio(
        "therapeutic_index",
        vec![
            ("td50", ParameterValue::Scalar(100.0)),
            ("ed50", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!((v - 10.0).abs() < 1e-8, "TI=TD50/ED50=10, got {v}");
}

#[test]
fn dose_response_hill() {
    let v = scalar(run_bio(
        "dose_response_hill",
        vec![
            ("dose", ParameterValue::Scalar(10.0)),
            ("dmax", ParameterValue::Scalar(100.0)),
            ("ec50", ParameterValue::Scalar(10.0)),
            ("n", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(
        (v - 50.0).abs() < 1e-6,
        "at dose=EC50, E=Dmax/2=50, got {v}"
    );
}
