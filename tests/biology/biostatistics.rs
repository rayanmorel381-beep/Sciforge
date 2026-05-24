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
fn odds_ratio() {
    let v = scalar(run_bio(
        "odds_ratio",
        vec![
            ("a", ParameterValue::Integer(20)),
            ("b", ParameterValue::Integer(80)),
            ("c", ParameterValue::Integer(10)),
            ("d", ParameterValue::Integer(90)),
        ],
    ));
    let expected = (20.0 * 90.0) / (80.0 * 10.0);
    assert!((v - expected).abs() < 1e-6, "OR=ad/bc=2.25, got {v}");
}

#[test]
fn sensitivity() {
    let v = scalar(run_bio(
        "sensitivity",
        vec![
            ("tp", ParameterValue::Integer(90)),
            ("fn_count", ParameterValue::Integer(10)),
        ],
    ));
    assert!((v - 0.9).abs() < 1e-8, "90/(90+10)=0.9, got {v}");
}

#[test]
fn specificity() {
    let v = scalar(run_bio(
        "specificity",
        vec![
            ("tn", ParameterValue::Integer(85)),
            ("fp", ParameterValue::Integer(15)),
        ],
    ));
    assert!((v - 0.85).abs() < 1e-8, "85/(85+15)=0.85, got {v}");
}

#[test]
fn number_needed_to_treat() {
    let v = scalar(run_bio(
        "number_needed_to_treat",
        vec![("arr", ParameterValue::Scalar(0.1))],
    ));
    assert!((v - 10.0).abs() < 1e-8, "1/0.1=10, got {v}");
}

#[test]
fn f1_score() {
    let v = scalar(run_bio(
        "f1_score",
        vec![
            ("tp", ParameterValue::Integer(50)),
            ("fp", ParameterValue::Integer(10)),
            ("fn_count", ParameterValue::Integer(5)),
        ],
    ));
    let precision = 50.0 / 60.0;
    let recall = 50.0 / 55.0;
    let expected = 2.0 * precision * recall / (precision + recall);
    assert!((v - expected).abs() < 1e-6, "F1 score, got {v}");
}
