use sciforge_hub::prelude::*;

fn run_maths(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Maths, name);
    for (k, v) in params {
        exp = exp.param(k, v);
    }
    ExperimentRunner::new()
        .run(&exp)
        .unwrap_or_else(|_| panic!("dispatch '{name}' failed"))
}

#[test]
fn heat_equation_1d_explicit() {
    let out = run_maths(
        "heat_equation_1d_explicit",
        vec![
            ("u", ParameterValue::Vector(vec![0.0, 100.0, 0.0])),
            ("alpha", ParameterValue::Scalar(0.01)),
            ("dx", ParameterValue::Scalar(1.0)),
            ("dt", ParameterValue::Scalar(0.1)),
        ],
    );
    match out {
        RunOutput::Vector(v) => assert_eq!(v.len(), 3, "output should have same size as input"),
        _ => panic!("expected Vector output, got {out:?}"),
    }
}

#[test]
fn wave_equation_1d() {
    let out = run_maths(
        "wave_equation_1d",
        vec![
            ("u", ParameterValue::Vector(vec![0.0, 1.0, 0.0])),
            ("u_prev", ParameterValue::Vector(vec![0.0, 1.0, 0.0])),
            ("c", ParameterValue::Scalar(1.0)),
            ("dx", ParameterValue::Scalar(1.0)),
            ("dt", ParameterValue::Scalar(0.1)),
        ],
    );
    match out {
        RunOutput::Vector(v) => assert!(!v.is_empty(), "wave equation should return data"),
        _ => panic!("expected Vector output, got {out:?}"),
    }
}
