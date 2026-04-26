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
fn ultrasensitivity_index() {
    let v = scalar(run_bio(
        "ultrasensitivity_index",
        vec![
            ("ec90", ParameterValue::Scalar(90.0)),
            ("ec10", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!(
        v > 0.0,
        "ultrasensitivity index should be positive, got {v}"
    );
}

#[test]
fn metabolic_control_coefficient() {
    let v = scalar(run_bio(
        "metabolic_control_coefficient",
        vec![
            ("flux_change", ParameterValue::Scalar(0.1)),
            ("flux_base", ParameterValue::Scalar(1.0)),
            ("enzyme_change", ParameterValue::Scalar(0.1)),
            ("enzyme_base", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((v - 1.0).abs() < 1e-8, "C=(dJ/J)/(dE/E)=1.0, got {v}");
}

#[test]
fn oscillation_period() {
    let v = scalar(run_bio(
        "oscillation_period",
        vec![
            (
                "time_series",
                ParameterValue::Vector(vec![0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0]),
            ),
            ("dt", ParameterValue::Scalar(0.25)),
        ],
    ));
    assert!(v > 0.0, "oscillation period should be positive, got {v}");
}
