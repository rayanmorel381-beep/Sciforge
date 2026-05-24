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
fn rate_constant_arrhenius() {
    let v = scalar(run_chem(
        "rate_constant_arrhenius",
        vec![
            ("a", ParameterValue::Scalar(1e13)),
            ("ea", ParameterValue::Scalar(75000.0)),
            ("t", ParameterValue::Scalar(298.15)),
        ],
    ));
    assert!(
        v > 0.0 && v.is_finite(),
        "rate constant should be positive, got {v}"
    );
}

#[test]
fn half_life_first_order() {
    let v = scalar(run_chem(
        "half_life_first_order",
        vec![("k", ParameterValue::Scalar(0.1))],
    ));
    let expected = (2.0_f64).ln() / 0.1;
    assert!((v - expected).abs() < 1e-6, "t1/2=ln2/k, got {v}");
}

#[test]
fn michaelis_menten() {
    let v = scalar(run_chem(
        "michaelis_menten",
        vec![
            ("vmax", ParameterValue::Scalar(100.0)),
            ("s", ParameterValue::Scalar(10.0)),
            ("km", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!((v - 50.0).abs() < 1e-8, "at S=Km, V=Vmax/2=50, got {v}");
}

#[test]
fn eyring_equation() {
    let v = scalar(run_chem(
        "eyring_equation",
        vec![
            ("kappa", ParameterValue::Scalar(1.0)),
            ("kb", ParameterValue::Scalar(1.381e-23)),
            ("t", ParameterValue::Scalar(298.15)),
            ("h", ParameterValue::Scalar(6.626e-34)),
            ("delta_g_dagger", ParameterValue::Scalar(75000.0)),
        ],
    ));
    assert!(v > 0.0 && v.is_finite(), "rate should be positive, got {v}");
}
