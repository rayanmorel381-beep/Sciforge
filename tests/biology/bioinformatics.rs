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

fn integer(o: RunOutput) -> i64 {
    match o {
        RunOutput::Integer(v) => v,
        _ => panic!("expected Integer, got {o:?}"),
    }
}

#[test]
fn edit_distance_identical() {
    let v = integer(run_bio(
        "edit_distance",
        vec![
            ("seq1", ParameterValue::Text("ACGT".to_string())),
            ("seq2", ParameterValue::Text("ACGT".to_string())),
        ],
    ));
    assert_eq!(v, 0, "identical sequences -> distance 0");
}

#[test]
fn alignment_gc_content() {
    let v = scalar(run_bio(
        "alignment_gc_content",
        vec![("seq", ParameterValue::Text("GCGC".to_string()))],
    ));
    assert!((v - 1.0).abs() < 1e-8, "GCGC is 100% GC, got {v}");
}

#[test]
fn sequence_identity() {
    let v = scalar(run_bio(
        "sequence_identity",
        vec![
            ("seq1", ParameterValue::Text("ACGT".to_string())),
            ("seq2", ParameterValue::Text("ACGT".to_string())),
        ],
    ));
    assert!(
        (v - 1.0).abs() < 1e-8,
        "identical => 100% identity, got {v}"
    );
}

#[test]
fn smith_waterman_score() {
    let v = integer(run_bio(
        "smith_waterman_score",
        vec![
            ("seq1", ParameterValue::Text("ACGT".to_string())),
            ("seq2", ParameterValue::Text("ACGT".to_string())),
            ("match_score", ParameterValue::Integer(2)),
            ("mismatch", ParameterValue::Integer(-1)),
            ("gap", ParameterValue::Integer(-2)),
        ],
    ));
    assert_eq!(v, 8, "4 matches * 2 = 8, got {v}");
}
