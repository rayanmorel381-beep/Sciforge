use sciforge::hub::prelude::*;

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
fn fermi_dirac() {
    let v = scalar(run_chem(
        "fermi_dirac",
        vec![
            ("energy", ParameterValue::Scalar(1.0)),
            ("fermi_level", ParameterValue::Scalar(1.0)),
            ("t", ParameterValue::Scalar(300.0)),
        ],
    ));
    assert!((v - 0.5).abs() < 1e-8, "at E=Ef, f=0.5, got {v}");
}

#[test]
fn band_gap_from_absorption() {
    let v = scalar(run_chem(
        "band_gap_from_absorption",
        vec![("wavelength_edge_nm", ParameterValue::Scalar(500.0))],
    ));
    assert!(v > 0.0, "band gap should be positive, got {v}");
}

#[test]
fn conductivity_semiconductor() {
    let v = scalar(run_chem(
        "conductivity_semiconductor",
        vec![
            ("n", ParameterValue::Scalar(1e16)),
            ("mu_e", ParameterValue::Scalar(0.14)),
            ("p", ParameterValue::Scalar(1e10)),
            ("mu_h", ParameterValue::Scalar(0.05)),
        ],
    ));
    assert!(v > 0.0, "conductivity should be positive, got {v}");
}
