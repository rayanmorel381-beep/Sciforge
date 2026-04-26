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
fn telomere_shortening() {
    let v = scalar(run_bio(
        "telomere_shortening",
        vec![
            ("initial_length", ParameterValue::Scalar(10000.0)),
            ("loss_per_division", ParameterValue::Scalar(50.0)),
            ("divisions", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!((v - 5000.0).abs() < 1e-6, "10000 - 50*100 = 5000, got {v}");
}

#[test]
fn gompertz_mortality_rate() {
    let v = scalar(run_bio(
        "gompertz_mortality_rate",
        vec![
            ("a", ParameterValue::Scalar(0.0001)),
            ("b", ParameterValue::Scalar(0.085)),
            ("age", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 0.0001).abs() < 1e-8, "at age=0, rate=a, got {v}");
}

#[test]
fn caloric_restriction_lifespan() {
    let v = scalar(run_bio(
        "caloric_restriction_lifespan",
        vec![
            ("base_lifespan", ParameterValue::Scalar(80.0)),
            ("restriction_fraction", ParameterValue::Scalar(0.3)),
            ("effect_coefficient", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(v > 80.0, "CR should extend lifespan, got {v}");
}

#[test]
fn mitochondrial_damage() {
    let v = scalar(run_bio(
        "mitochondrial_damage",
        vec![
            ("intact_fraction", ParameterValue::Scalar(1.0)),
            ("damage_rate", ParameterValue::Scalar(0.01)),
            ("dt", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(
        v < 1.0 && v > 0.0,
        "intact fraction should decrease, got {v}"
    );
}

#[test]
fn ros_steady_state() {
    let v = scalar(run_bio(
        "ros_steady_state",
        vec![
            ("production_rate", ParameterValue::Scalar(10.0)),
            ("sod_activity", ParameterValue::Scalar(3.0)),
            ("catalase_activity", ParameterValue::Scalar(2.0)),
        ],
    ));
    assert!(
        v > 0.0 && v.is_finite(),
        "steady state ROS should be positive, got {v}"
    );
}
