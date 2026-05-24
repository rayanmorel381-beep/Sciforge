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
fn firing_rate() {
    let v = scalar(run_bio(
        "firing_rate",
        vec![
            (
                "spikes",
                ParameterValue::Vector(vec![1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0]),
            ),
            ("dt", ParameterValue::Scalar(0.001)),
            ("total_steps", ParameterValue::Integer(10)),
        ],
    ));
    assert!(v > 0.0, "firing rate should be positive, got {v}");
}

#[test]
fn fano_factor() {
    let v = scalar(run_bio(
        "fano_factor",
        vec![(
            "spike_counts",
            ParameterValue::Vector(vec![5.0, 5.0, 5.0, 5.0]),
        )],
    ));
    assert!((v - 0.0).abs() < 1e-8, "constant counts => Fano=0, got {v}");
}

#[test]
fn rescorla_wagner_update() {
    let v = scalar(run_bio(
        "rescorla_wagner_update",
        vec![
            ("value", ParameterValue::Scalar(0.0)),
            ("reward", ParameterValue::Scalar(1.0)),
            ("alpha", ParameterValue::Scalar(0.1)),
        ],
    ));
    assert!(
        (v - 0.1).abs() < 1e-8,
        "dV=alpha*(reward-value)=0.1, got {v}"
    );
}

#[test]
fn stdp_weight_change() {
    let v = scalar(run_bio(
        "stdp_weight_change",
        vec![
            ("delta_t_ms", ParameterValue::Scalar(0.0)),
            ("a_plus", ParameterValue::Scalar(0.01)),
            ("a_minus", ParameterValue::Scalar(0.01)),
            ("tau_plus", ParameterValue::Scalar(20.0)),
            ("tau_minus", ParameterValue::Scalar(20.0)),
        ],
    ));
    assert!(v.abs() < 0.02, "at dt=0, weight change near 0, got {v}");
}
