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

fn scalar(o: RunOutput) -> f64 {
    match o {
        RunOutput::Scalar(v) => v,
        _ => panic!("expected Scalar, got {o:?}"),
    }
}

#[test]
fn low_pass_rc() {
    let o = run_maths(
        "low_pass_rc",
        vec![
            ("signal", ParameterValue::Vector(vec![1.0, 1.0, 1.0, 1.0])),
            ("dt", ParameterValue::Scalar(0.001)),
            ("rc", ParameterValue::Scalar(0.001)),
        ],
    );
    match o {
        RunOutput::Vector(v) => assert!(!v.is_empty(), "filtered signal should be non-empty"),
        other => panic!("expected Vector, got {other:?}"),
    }
}

#[test]
fn butterworth_gain() {
    let v = scalar(run_maths(
        "butterworth_gain",
        vec![
            ("freq", ParameterValue::Scalar(0.0)),
            ("cutoff", ParameterValue::Scalar(1000.0)),
            ("order", ParameterValue::Integer(2)),
        ],
    ));
    assert!((v - 1.0).abs() < 1e-6, "Butterworth gain at DC=1, got {v}");
}

#[test]
fn kalman_filter_1d() {
    let o = run_maths(
        "kalman_filter_1d",
        vec![
            (
                "measurements",
                ParameterValue::Vector(vec![10.0, 10.5, 9.8, 10.2]),
            ),
            ("process_noise", ParameterValue::Scalar(0.1)),
            ("measurement_noise", ParameterValue::Scalar(1.0)),
            ("initial_estimate", ParameterValue::Scalar(8.0)),
        ],
    );
    match o {
        RunOutput::Vector(v) => assert!(!v.is_empty(), "kalman output should be non-empty"),
        other => panic!("expected Vector, got {other:?}"),
    }
}
