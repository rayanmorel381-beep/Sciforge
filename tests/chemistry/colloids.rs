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
fn stokes_sedimentation() {
    let v = scalar(run_chem(
        "stokes_sedimentation",
        vec![
            ("r", ParameterValue::Scalar(1e-6)),
            ("rho_p", ParameterValue::Scalar(2500.0)),
            ("rho_f", ParameterValue::Scalar(1000.0)),
            ("viscosity", ParameterValue::Scalar(1e-3)),
        ],
    ));
    assert!(
        v > 0.0,
        "sedimentation velocity should be positive, got {v}"
    );
}

#[test]
fn brownian_diffusion_coefficient() {
    let v = scalar(run_chem(
        "brownian_diffusion_coefficient",
        vec![
            ("t", ParameterValue::Scalar(298.0)),
            ("viscosity", ParameterValue::Scalar(1e-3)),
            ("r", ParameterValue::Scalar(1e-6)),
        ],
    ));
    assert!(v > 0.0, "diffusion coefficient should be positive, got {v}");
}

#[test]
fn zeta_potential_smoluchowski() {
    let v = scalar(run_chem(
        "zeta_potential_smoluchowski",
        vec![
            ("mobility", ParameterValue::Scalar(3e-8)),
            ("viscosity", ParameterValue::Scalar(1e-3)),
            ("epsilon", ParameterValue::Scalar(7.08e-10)),
        ],
    ));
    assert!(v.is_finite(), "zeta potential should be finite, got {v}");
}
