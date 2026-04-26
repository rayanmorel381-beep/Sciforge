use sciforge::hub::prelude::*;

fn run(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Biophysics, name);
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
fn stokes_drag_force() {
    let v = scalar(run(
        "stokes_drag_force",
        vec![
            ("viscosity", ParameterValue::Scalar(1e-3)),
            ("radius", ParameterValue::Scalar(1e-6)),
            ("velocity", ParameterValue::Scalar(1e-4)),
        ],
    ));
    assert!(v > 0.0, "drag force should be positive, got {v}");
}

#[test]
fn reynolds_number() {
    let v = scalar(run(
        "reynolds_number",
        vec![
            ("density", ParameterValue::Scalar(1000.0)),
            ("velocity", ParameterValue::Scalar(1.0)),
            ("length", ParameterValue::Scalar(0.01)),
            ("viscosity", ParameterValue::Scalar(1e-3)),
        ],
    ));
    assert!((v - 10000.0).abs() < 1.0, "Re=ρvL/μ=10000, got {v}");
}

#[test]
fn fick_diffusion_flux() {
    let v = scalar(run(
        "fick_diffusion_flux",
        vec![
            ("diffusion_coeff", ParameterValue::Scalar(1e-9)),
            ("concentration_gradient", ParameterValue::Scalar(1e6)),
        ],
    ));
    assert!((v - (-1e-3)).abs() < 1e-6, "J=-D*dC/dx, got {v}");
}

#[test]
fn electrophoretic_mobility() {
    let v = scalar(run(
        "electrophoretic_mobility",
        vec![
            ("charge", ParameterValue::Scalar(1.6e-19)),
            ("friction_coefficient", ParameterValue::Scalar(1e-12)),
        ],
    ));
    assert!(v > 0.0, "mobility should be positive, got {v}");
}
