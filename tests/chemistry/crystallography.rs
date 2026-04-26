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
fn bragg_law() {
    let v = scalar(run_chem(
        "bragg_law",
        vec![
            ("d", ParameterValue::Scalar(2.0)),
            ("theta", ParameterValue::Scalar(std::f64::consts::FRAC_PI_6)),
            ("n", ParameterValue::Integer(1)),
        ],
    ));
    assert!(v > 0.0, "wavelength should be positive, got {v}");
}

#[test]
fn interplanar_spacing_cubic() {
    let v = scalar(run_chem(
        "interplanar_spacing_cubic",
        vec![
            ("a", ParameterValue::Scalar(5.0)),
            ("h", ParameterValue::Scalar(1.0)),
            ("k", ParameterValue::Scalar(0.0)),
            ("l", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 5.0).abs() < 1e-8, "d(100)=a=5, got {v}");
}

#[test]
fn scherrer_crystal_size() {
    let v = scalar(run_chem(
        "scherrer_crystal_size",
        vec![
            ("k", ParameterValue::Scalar(0.9)),
            ("wavelength", ParameterValue::Scalar(1.54)),
            ("fwhm", ParameterValue::Scalar(0.5)),
            ("theta", ParameterValue::Scalar(0.2618)),
        ],
    ));
    assert!(v > 0.0, "crystal size should be positive, got {v}");
}
