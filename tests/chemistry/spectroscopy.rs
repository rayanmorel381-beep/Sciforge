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
fn wavenumber_to_wavelength() {
    let v = scalar(run_chem(
        "wavenumber_to_wavelength",
        vec![("wavenumber_cm", ParameterValue::Scalar(1000.0))],
    ));
    assert!(v > 0.0, "wavelength should be positive, got {v}");
}

#[test]
fn larmor_frequency() {
    let v = scalar(run_chem(
        "larmor_frequency",
        vec![
            ("gamma", ParameterValue::Scalar(42.577e6)),
            ("b0", ParameterValue::Scalar(1.5)),
        ],
    ));
    assert!(v > 0.0, "Larmor freq should be positive, got {v}");
}

#[test]
fn chemical_shift_ppm() {
    let v = scalar(run_chem(
        "chemical_shift_ppm",
        vec![
            ("frequency_sample", ParameterValue::Scalar(300.001e6)),
            ("frequency_reference", ParameterValue::Scalar(300.0e6)),
            ("spectrometer", ParameterValue::Scalar(300.0e6)),
        ],
    ));
    assert!(v > 0.0, "shift should be positive, got {v}");
}

#[test]
fn reduced_mass() {
    let v = scalar(run_chem(
        "reduced_mass",
        vec![
            ("m1", ParameterValue::Scalar(1.0)),
            ("m2", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((v - 0.5).abs() < 1e-8, "mu=m1*m2/(m1+m2)=0.5, got {v}");
}
