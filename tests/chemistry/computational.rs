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
fn thomas_fermi_kinetic_energy() {
    let v = scalar(run_chem(
        "thomas_fermi_kinetic_energy",
        vec![
            ("density", ParameterValue::Vector(vec![1.0])),
            ("volume_element", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(v > 0.0, "kinetic energy should be positive, got {v}");
}

#[test]
fn gaussian_primitive() {
    let v = scalar(run_chem(
        "gaussian_primitive",
        vec![
            ("alpha", ParameterValue::Scalar(1.0)),
            ("r_sq", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 1.0).abs() < 1e-6, "at r=0, exp(0)=1, got {v}");
}

#[test]
fn dft_hartree_energy() {
    let v = scalar(run_chem(
        "dft_hartree_energy",
        vec![
            ("density", ParameterValue::Vector(vec![0.5])),
            ("potential", ParameterValue::Vector(vec![2.0])),
            ("volume_element", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(v.is_finite(), "Hartree energy should be finite, got {v}");
}
