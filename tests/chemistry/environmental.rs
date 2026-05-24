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
fn henry_law_solubility() {
    let v = scalar(run_chem(
        "henry_law_solubility",
        vec![
            ("kh", ParameterValue::Scalar(0.034)),
            ("partial_pressure", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((v - 0.034).abs() < 1e-6, "C=Kh*P=0.034, got {v}");
}

#[test]
fn biochemical_oxygen_demand() {
    let v = scalar(run_chem(
        "biochemical_oxygen_demand",
        vec![
            ("bod_ultimate", ParameterValue::Scalar(200.0)),
            ("k", ParameterValue::Scalar(0.23)),
            ("t", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!(v.abs() < 1e-6, "at t=0, BOD=0, got {v}");
}

#[test]
fn dissolved_oxygen_saturation() {
    let v = scalar(run_chem(
        "dissolved_oxygen_saturation",
        vec![("t", ParameterValue::Scalar(20.0))],
    ));
    assert!(v > 5.0 && v < 15.0, "DO sat at 20°C ~9 mg/L, got {v}");
}
