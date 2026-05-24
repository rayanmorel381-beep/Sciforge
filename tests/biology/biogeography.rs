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
fn range_size_latitude() {
    let v = scalar(run_bio(
        "range_size_latitude",
        vec![("area", ParameterValue::Scalar(1000.0))],
    ));
    assert!(
        v.is_finite() && v > 0.0,
        "range size should be positive, got {v}"
    );
}

#[test]
fn latitudinal_diversity_gradient() {
    let v = scalar(run_bio(
        "latitudinal_diversity_gradient",
        vec![
            ("latitude", ParameterValue::Scalar(0.0)),
            ("max_richness", ParameterValue::Scalar(500.0)),
            ("steepness", ParameterValue::Scalar(0.05)),
        ],
    ));
    assert!(
        (v - 500.0).abs() < 1e-6,
        "at equator, richness=max, got {v}"
    );
}

#[test]
fn range_shift_velocity() {
    let v = scalar(run_bio(
        "range_shift_velocity",
        vec![
            ("temp_change_rate", ParameterValue::Scalar(0.03)),
            ("spatial_temp_gradient", ParameterValue::Scalar(0.006)),
        ],
    ));
    assert!((v - 5.0).abs() < 1e-6, "0.03/0.006=5, got {v}");
}

#[test]
fn wallace_line_effect() {
    let v = scalar(run_bio(
        "wallace_line_effect",
        vec![
            ("dispersal_ability", ParameterValue::Scalar(10.0)),
            ("barrier_width", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!(
        (0.0..=1.0).contains(&v),
        "effect should be in [0,1], got {v}"
    );
}
