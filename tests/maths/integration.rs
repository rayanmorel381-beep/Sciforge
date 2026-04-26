use sciforge::hub::prelude::*;

fn run_maths(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Maths, name);
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
fn halton_sequence_integration() {
    let v = scalar(run_maths(
        "halton_sequence_integration",
        vec![
            ("index", ParameterValue::Integer(10)),
            ("base", ParameterValue::Integer(2)),
        ],
    ));
    assert!(
        v.is_finite(),
        "integration result should be finite, got {v}"
    );
}
