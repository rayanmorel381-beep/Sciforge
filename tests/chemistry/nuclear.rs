use sciforge_hub::prelude::*;

fn run_chem(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Chemistry, name);
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
fn half_life_to_decay_constant() {
    let v = scalar(run_chem(
        "half_life_to_decay_constant",
        vec![("half_life", ParameterValue::Scalar(5730.0))],
    ));
    let expected = (2.0_f64).ln() / 5730.0;
    assert!((v - expected).abs() < 1e-10, "lambda=ln2/t1/2, got {v}");
}

#[test]
fn binding_energy_per_nucleon() {
    let v = scalar(run_chem(
        "binding_energy_per_nucleon",
        vec![
            ("binding_energy", ParameterValue::Scalar(492.26)),
            ("a", ParameterValue::Scalar(56.0)),
        ],
    ));
    assert!((v - 8.79).abs() < 0.01, "BE/A=492.26/56≈8.79, got {v}");
}

#[test]
fn q_value() {
    let v = scalar(run_chem(
        "q_value",
        vec![
            ("reactant_masses", ParameterValue::Vector(vec![4.0330])),
            ("product_masses", ParameterValue::Vector(vec![4.0320])),
        ],
    ));
    assert!(v > 0.0, "positive Q => exothermic, got {v}");
}
