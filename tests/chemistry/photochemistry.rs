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
fn photon_energy() {
    let v = scalar(run_chem(
        "photon_energy",
        vec![("wavelength_nm", ParameterValue::Scalar(500.0))],
    ));
    assert!(v > 0.0, "photon energy should be positive, got {v}");
}

#[test]
fn quantum_yield() {
    let v = scalar(run_chem(
        "quantum_yield",
        vec![
            ("molecules_reacted", ParameterValue::Scalar(50.0)),
            ("photons_absorbed", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!((v - 0.5).abs() < 1e-8, "QY=50/100=0.5, got {v}");
}

#[test]
fn stern_volmer() {
    let v = scalar(run_chem(
        "stern_volmer",
        vec![
            ("i0", ParameterValue::Scalar(100.0)),
            ("ksv", ParameterValue::Scalar(10.0)),
            ("quencher", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 100.0).abs() < 1e-6, "no quencher => I=I0, got {v}");
}

#[test]
fn fret_efficiency() {
    let v = scalar(run_chem(
        "fret_efficiency",
        vec![
            ("r", ParameterValue::Scalar(5.0)),
            ("r0", ParameterValue::Scalar(5.0)),
        ],
    ));
    assert!((v - 0.5).abs() < 1e-8, "at R=R0, E=0.5, got {v}");
}
