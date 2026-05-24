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
fn self_renewal_probability() {
    let v = scalar(run_bio(
        "self_renewal_probability",
        vec![
            ("symmetric_rate", ParameterValue::Scalar(0.3)),
            ("total_division_rate", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((0.0..=1.0).contains(&v), "probability in [0,1], got {v}");
}

#[test]
fn reprogramming_efficiency() {
    let v = scalar(run_bio(
        "reprogramming_efficiency",
        vec![
            ("oct4", ParameterValue::Scalar(1.0)),
            ("sox2", ParameterValue::Scalar(1.0)),
            ("klf4", ParameterValue::Scalar(1.0)),
            ("myc", ParameterValue::Scalar(1.0)),
            ("epigenetic_barrier", ParameterValue::Scalar(0.5)),
        ],
    ));
    assert!((0.0..=1.0).contains(&v), "efficiency in [0,1], got {v}");
}

#[test]
fn niche_occupancy() {
    let v = scalar(run_bio(
        "niche_occupancy",
        vec![
            ("stem_cells", ParameterValue::Scalar(50.0)),
            ("niche_capacity", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!((v - 0.5).abs() < 1e-8, "occupancy=50/100=0.5, got {v}");
}
