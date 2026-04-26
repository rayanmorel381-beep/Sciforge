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
fn fertilization_probability() {
    let v = scalar(run_bio(
        "fertilization_probability",
        vec![
            ("sperm_count", ParameterValue::Scalar(1e8)),
            ("half_max", ParameterValue::Scalar(1e8)),
        ],
    ));
    assert!(
        (v - 0.5).abs() < 1e-6,
        "at sperm=half_max, prob=0.5, got {v}"
    );
}

#[test]
fn hcg_doubling() {
    let v = scalar(run_bio(
        "hcg_doubling",
        vec![
            ("initial", ParameterValue::Scalar(100.0)),
            ("doubling_time", ParameterValue::Scalar(48.0)),
            ("t", ParameterValue::Scalar(48.0)),
        ],
    ));
    assert!(
        (v - 200.0).abs() < 1e-6,
        "at t=Td, hCG doubles to 200, got {v}"
    );
}

#[test]
fn sperm_motility_score() {
    let v = scalar(run_bio(
        "sperm_motility_score",
        vec![
            ("progressive", ParameterValue::Scalar(50.0)),
            ("non_progressive", ParameterValue::Scalar(20.0)),
            ("immotile", ParameterValue::Scalar(30.0)),
        ],
    ));
    assert!((0.0..=1.0).contains(&v), "motility score in [0,1], got {v}");
}

#[test]
fn ivf_success_rate() {
    let v = scalar(run_bio(
        "ivf_success_rate",
        vec![
            ("age", ParameterValue::Scalar(30.0)),
            ("embryo_quality", ParameterValue::Scalar(0.8)),
            ("endometrial_thickness", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!((0.0..=1.0).contains(&v), "IVF success in [0,1], got {v}");
}
