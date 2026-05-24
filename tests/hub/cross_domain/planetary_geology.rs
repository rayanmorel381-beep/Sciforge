use sciforge_hub::prelude::*;

fn run(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::PlanetaryGeology, name);
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
fn impact_energy() {
    let v = scalar(run(
        "impact_energy",
        vec![
            ("projectile_mass", ParameterValue::Scalar(1e6)),
            ("impact_velocity", ParameterValue::Scalar(2e4)),
        ],
    ));
    let expected = 0.5 * 1e6 * (2e4_f64).powi(2);
    assert!(
        (v - expected).abs() / expected < 1e-6,
        "E=½mv²={expected}, got {v}"
    );
}

#[test]
fn surface_temperature_equilibrium() {
    let v = scalar(run(
        "surface_temperature_equilibrium",
        vec![
            ("solar_flux", ParameterValue::Scalar(1361.0)),
            ("albedo", ParameterValue::Scalar(0.3)),
            ("emissivity", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((v - 255.0).abs() < 5.0, "Earth Teq ≈ 255 K, got {v}");
}

#[test]
fn crater_counting_surface_age() {
    let v = scalar(run(
        "crater_counting_surface_age",
        vec![
            ("crater_density", ParameterValue::Scalar(1e3)),
            ("production_rate", ParameterValue::Scalar(1e-6)),
        ],
    ));
    assert!((v - 1e9).abs() / 1e9 < 1e-6, "age=N/rate=1e9, got {v}");
}

#[test]
fn volcanic_effusion_rate() {
    let v = scalar(run(
        "volcanic_effusion_rate",
        vec![
            ("thermal_flux", ParameterValue::Scalar(1e9)),
            ("specific_heat", ParameterValue::Scalar(1000.0)),
            ("delta_t", ParameterValue::Scalar(200.0)),
            ("latent_heat", ParameterValue::Scalar(4e5)),
        ],
    ));
    assert!(v > 0.0, "effusion rate should be positive, got {v}");
}
