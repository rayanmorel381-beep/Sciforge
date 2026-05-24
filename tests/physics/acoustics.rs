use sciforge_hub::prelude::*;

fn run_phys(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Physics, name);
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
fn speed_of_sound_gas() {
    let v = scalar(run_phys(
        "speed_of_sound_gas",
        vec![
            ("gamma", ParameterValue::Scalar(1.4)),
            ("r", ParameterValue::Scalar(8.314)),
            ("t", ParameterValue::Scalar(293.15)),
            ("m", ParameterValue::Scalar(0.029)),
        ],
    ));
    assert!((v - 343.0).abs() < 10.0, "speed of sound ~343 m/s, got {v}");
}

#[test]
fn doppler_approaching() {
    let v = scalar(run_phys(
        "doppler_approaching",
        vec![
            ("f0", ParameterValue::Scalar(440.0)),
            ("v_source", ParameterValue::Scalar(0.0)),
            ("c", ParameterValue::Scalar(343.0)),
        ],
    ));
    assert!((v - 440.0).abs() < 1e-6, "stationary => f=f0, got {v}");
}

#[test]
fn reverberation_time_sabine() {
    let v = scalar(run_phys(
        "reverberation_time_sabine",
        vec![
            ("v", ParameterValue::Scalar(500.0)),
            ("a", ParameterValue::Scalar(50.0)),
        ],
    ));
    assert!(v > 0.0, "RT60 should be positive, got {v}");
}

#[test]
fn intensity_level_db() {
    let v = scalar(run_phys(
        "intensity_level_db",
        vec![
            ("intensity", ParameterValue::Scalar(1e-12)),
            ("i_ref", ParameterValue::Scalar(1e-12)),
        ],
    ));
    assert!(v.abs() < 1e-6, "threshold of hearing = 0 dB, got {v}");
}
