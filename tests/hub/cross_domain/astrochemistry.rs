use sciforge::hub::prelude::*;

fn run(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Astrochemistry, name);
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
fn jeans_mass() {
    let v = scalar(run(
        "jeans_mass",
        vec![
            ("temperature", ParameterValue::Scalar(10.0)),
            ("number_density", ParameterValue::Scalar(1e-18)),
            ("mean_molecular_weight", ParameterValue::Scalar(2.33)),
        ],
    ));
    assert!(v > 0.0, "Jeans mass should be positive, got {v}");
}

#[test]
fn freefall_time() {
    let v = scalar(run(
        "freefall_time",
        vec![
            ("number_density", ParameterValue::Scalar(1e-18)),
            ("mean_molecular_weight", ParameterValue::Scalar(2.33)),
        ],
    ));
    assert!(v > 0.0, "freefall time should be positive, got {v}");
}

#[test]
fn stroemgren_radius() {
    let v = scalar(run(
        "stroemgren_radius",
        vec![
            ("ionizing_photon_rate", ParameterValue::Scalar(1e49)),
            ("hydrogen_density", ParameterValue::Scalar(1e4)),
            ("recombination_coeff", ParameterValue::Scalar(2.6e-13)),
        ],
    ));
    assert!(v > 0.0, "Stroemgren radius should be positive, got {v}");
}

#[test]
fn dust_equilibrium_temperature() {
    let v = scalar(run(
        "dust_equilibrium_temperature",
        vec![
            ("luminosity", ParameterValue::Scalar(3.828e26)),
            ("distance", ParameterValue::Scalar(1.496e11)),
            ("grain_radius", ParameterValue::Scalar(1e-7)),
            ("absorption_efficiency", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(v > 100.0 && v < 500.0, "dust T near 1 AU ~ 278K, got {v}");
}
