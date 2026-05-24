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
fn hyphal_growth_rate() {
    let v = scalar(run_bio(
        "hyphal_growth_rate",
        vec![
            ("tip_extension", ParameterValue::Scalar(5.0)),
            ("branching_rate", ParameterValue::Scalar(0.1)),
            ("tips", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!(v > 0.0, "growth rate should be positive, got {v}");
}

#[test]
fn spore_germination_probability() {
    let v = scalar(run_bio(
        "spore_germination_probability",
        vec![
            ("water_activity", ParameterValue::Scalar(0.95)),
            ("temperature", ParameterValue::Scalar(25.0)),
            ("aw_min", ParameterValue::Scalar(0.7)),
            ("t_min", ParameterValue::Scalar(5.0)),
            ("t_max", ParameterValue::Scalar(40.0)),
        ],
    ));
    assert!(v > 0.0 && v <= 1.0, "probability in (0,1], got {v}");
}

#[test]
fn decomposition_rate() {
    let v = scalar(run_bio(
        "mycology_decomposition_rate",
        vec![
            ("k", ParameterValue::Scalar(0.05)),
            ("mass", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!(v > 0.0, "decomposition rate should be positive, got {v}");
}

#[test]
fn carbon_use_efficiency() {
    let v = scalar(run_bio(
        "mycology_carbon_use_efficiency",
        vec![
            ("co2_respired", ParameterValue::Scalar(20.0)),
            ("carbon_assimilated", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!(v > 0.0 && v <= 1.0, "CUE in (0,1], got {v}");
}
