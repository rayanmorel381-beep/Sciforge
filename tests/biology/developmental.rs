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
fn morphogen_gradient_steady() {
    let v = scalar(run_bio(
        "morphogen_gradient_steady",
        vec![
            ("source", ParameterValue::Scalar(100.0)),
            ("decay", ParameterValue::Scalar(0.1)),
            ("diffusion", ParameterValue::Scalar(1.0)),
            ("x", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!(
        (v - 100.0).abs() < 1e-6,
        "at x=0, concentration=source, got {v}"
    );
}

#[test]
fn french_flag_model() {
    let o = run_bio(
        "french_flag_model",
        vec![
            ("concentration", ParameterValue::Scalar(50.0)),
            ("t1", ParameterValue::Scalar(75.0)),
            ("t2", ParameterValue::Scalar(25.0)),
        ],
    );
    match o {
        RunOutput::Integer(v) => assert!(v >= 0, "cell fate index >= 0, got {v}"),
        other => panic!("expected Integer, got {other:?}"),
    }
}

#[test]
fn lateral_inhibition() {
    let o = run_bio(
        "lateral_inhibition",
        vec![
            ("cells", ParameterValue::Vector(vec![1.0, 0.5, 0.8])),
            ("notch", ParameterValue::Vector(vec![0.0, 0.0, 0.0])),
            ("delta", ParameterValue::Vector(vec![0.5, 0.5, 0.5])),
            ("beta_n", ParameterValue::Scalar(1.0)),
            ("beta_d", ParameterValue::Scalar(1.0)),
            ("k", ParameterValue::Scalar(0.5)),
            ("n", ParameterValue::Scalar(2.0)),
            ("dt", ParameterValue::Scalar(0.01)),
            ("steps", ParameterValue::Scalar(10.0)),
        ],
    );
    match o {
        RunOutput::Vector(v) => assert!(!v.is_empty(), "should return updated state"),
        other => panic!("expected Vector, got {other:?}"),
    }
}
