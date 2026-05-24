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
fn nernst_potential() {
    let v = scalar(run_chem(
        "nernst_potential",
        vec![
            ("e0", ParameterValue::Scalar(0.34)),
            ("n", ParameterValue::Scalar(2.0)),
            ("q", ParameterValue::Scalar(1.0)),
            ("t", ParameterValue::Scalar(298.15)),
        ],
    ));
    assert!(v.is_finite(), "Nernst potential should be finite, got {v}");
}

#[test]
fn faraday_mass() {
    let v = scalar(run_chem(
        "faraday_mass",
        vec![
            ("i", ParameterValue::Scalar(10.0)),
            ("t", ParameterValue::Scalar(3600.0)),
            ("m", ParameterValue::Scalar(63.55)),
            ("n", ParameterValue::Scalar(2.0)),
        ],
    ));
    assert!(v > 0.0, "deposited mass should be positive, got {v}");
}

#[test]
fn butler_volmer() {
    let v = scalar(run_chem(
        "butler_volmer",
        vec![
            ("i0", ParameterValue::Scalar(0.01)),
            ("alpha_a", ParameterValue::Scalar(0.5)),
            ("alpha_c", ParameterValue::Scalar(0.5)),
            ("eta", ParameterValue::Scalar(0.0)),
            ("t", ParameterValue::Scalar(298.15)),
        ],
    ));
    assert!(v.abs() < 1e-10, "at zero overpotential, i=0, got {v}");
}

#[test]
fn cell_potential() {
    let v = scalar(run_chem(
        "cell_potential",
        vec![
            ("e_cathode", ParameterValue::Scalar(0.34)),
            ("e_anode", ParameterValue::Scalar(-0.76)),
        ],
    ));
    assert!((v - 1.10).abs() < 1e-8, "Ecell=0.34-(-0.76)=1.10, got {v}");
}
