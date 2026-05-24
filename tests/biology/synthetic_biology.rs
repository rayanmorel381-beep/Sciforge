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
fn hill_activation() {
    let v = scalar(run_bio(
        "hill_activation",
        vec![
            ("x", ParameterValue::Scalar(10.0)),
            ("k", ParameterValue::Scalar(10.0)),
            ("n", ParameterValue::Scalar(2.0)),
        ],
    ));
    assert!((v - 0.5).abs() < 1e-8, "at x=K, activation=0.5, got {v}");
}

#[test]
fn hill_repression() {
    let v = scalar(run_bio(
        "hill_repression",
        vec![
            ("x", ParameterValue::Scalar(10.0)),
            ("k", ParameterValue::Scalar(10.0)),
            ("n", ParameterValue::Scalar(2.0)),
        ],
    ));
    assert!((v - 0.5).abs() < 1e-8, "at x=K, repression=0.5, got {v}");
}

#[test]
fn and_gate() {
    let v = scalar(run_bio(
        "and_gate",
        vec![
            ("input_a", ParameterValue::Scalar(100.0)),
            ("input_b", ParameterValue::Scalar(100.0)),
            ("k_a", ParameterValue::Scalar(10.0)),
            ("k_b", ParameterValue::Scalar(10.0)),
            ("n", ParameterValue::Scalar(2.0)),
            ("max_output", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(v > 0.9, "both inputs high => output high, got {v}");
}

#[test]
fn crispr_on_target_score() {
    let v = scalar(run_bio(
        "crispr_on_target_score",
        vec![
            ("gc_content", ParameterValue::Scalar(0.5)),
            (
                "position_scores",
                ParameterValue::Vector(vec![1.0, 1.0, 1.0, 1.0, 1.0]),
            ),
        ],
    ));
    assert!(v > 0.0 && v <= 1.0, "score in (0,1], got {v}");
}
