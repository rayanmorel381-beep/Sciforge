use sciforge::hub::prelude::*;

fn run(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::AtmosphericChemistry, name);
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
fn photolysis_rate() {
    let v = scalar(run(
        "photolysis_rate",
        vec![
            ("cross_section", ParameterValue::Scalar(1e-20)),
            ("quantum_yield", ParameterValue::Scalar(1.0)),
            ("actinic_flux", ParameterValue::Scalar(1e15)),
        ],
    ));
    assert!(v > 0.0, "photolysis rate should be positive, got {v}");
}

#[test]
fn henry_law_concentration() {
    let v = scalar(run(
        "henry_law_concentration",
        vec![
            ("henry_constant", ParameterValue::Scalar(3.4e-2)),
            ("partial_pressure", ParameterValue::Scalar(4e-4)),
        ],
    ));
    assert!(
        v > 0.0,
        "dissolved concentration should be positive, got {v}"
    );
}

#[test]
fn column_density() {
    let v = scalar(run(
        "column_density",
        vec![
            ("number_density", ParameterValue::Scalar(2.5e19)),
            ("path_length", ParameterValue::Scalar(1000.0)),
        ],
    ));
    assert!((v - 2.5e22).abs() < 1e18, "N = n * L, got {v}");
}

#[test]
fn aerosol_optical_depth() {
    let v = scalar(run(
        "aerosol_optical_depth",
        vec![
            ("extinction_coeff", ParameterValue::Scalar(1e-4)),
            ("layer_thickness", ParameterValue::Scalar(10000.0)),
        ],
    ));
    assert!((v - 1.0).abs() < 1e-6, "AOD = β * L = 1.0, got {v}");
}
