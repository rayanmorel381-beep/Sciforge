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
fn jukes_cantor_distance() {
    let v = scalar(run_bio(
        "jukes_cantor_distance",
        vec![("p_distance", ParameterValue::Scalar(0.0))],
    ));
    assert!((v - 0.0).abs() < 1e-8, "p=0 => d=0, got {v}");
}

#[test]
fn parsimony_score() {
    let o = run_bio(
        "parsimony_score",
        vec![
            (
                "sequences_flat",
                ParameterValue::Vector(vec![0.0, 1.0, 0.0, 1.0, 1.0, 0.0]),
            ),
            ("seq_len", ParameterValue::Integer(2)),
            ("tree", ParameterValue::Vector(vec![0.0, 1.0, 1.0, 2.0])),
        ],
    );
    match o {
        RunOutput::Integer(v) => assert!(v >= 0, "parsimony score >= 0, got {v}"),
        other => panic!("expected Integer, got {other:?}"),
    }
}

#[test]
fn sackin_index() {
    let v = scalar(run_bio(
        "sackin_index",
        vec![(
            "branch_depths",
            ParameterValue::Vector(vec![2.0, 2.0, 3.0, 3.0]),
        )],
    ));
    assert!((v - 10.0).abs() < 1e-8, "sum=2+2+3+3=10, got {v}");
}
