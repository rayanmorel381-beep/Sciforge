use sciforge::hub::prelude::*;

fn run_astro(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Astronomy, name);
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
fn luminosity_from_radius_temp() {
    let v = scalar(run_astro(
        "luminosity_from_radius_temp",
        vec![
            ("r", ParameterValue::Scalar(6.957e8)),
            ("t", ParameterValue::Scalar(5778.0)),
        ],
    ));
    let solar_l = 3.828e26;
    assert!(
        (v - solar_l).abs() / solar_l < 0.02,
        "Sun luminosity, got {v}"
    );
}

#[test]
fn stefan_boltzmann_blackbody() {
    let l1 = scalar(run_astro(
        "luminosity_from_radius_temp",
        vec![
            ("r", ParameterValue::Scalar(1.0)),
            ("t", ParameterValue::Scalar(1000.0)),
        ],
    ));
    let l2 = scalar(run_astro(
        "luminosity_from_radius_temp",
        vec![
            ("r", ParameterValue::Scalar(1.0)),
            ("t", ParameterValue::Scalar(2000.0)),
        ],
    ));
    assert!((l2 / l1 - 16.0).abs() < 0.1, "L ∝ T⁴ → ×16 for ×2 T");
}

#[test]
fn wien_peak_wavelength() {
    let v = scalar(run_astro(
        "wien_peak_wavelength",
        vec![("temperature", ParameterValue::Scalar(5778.0))],
    ));
    assert!(v > 4e-7 && v < 6e-7, "Sun peak ≈ 500nm, got {v}");
}

#[test]
fn schwarzschild_radius() {
    let v = scalar(run_astro(
        "schwarzschild_radius",
        vec![("mass", ParameterValue::Scalar(1.989e30))],
    ));
    assert!(
        (v - 2953.0).abs() < 10.0,
        "Solar Schwarzschild ≈ 2953m, got {v}"
    );
}

#[test]
fn chandrasekhar_limit() {
    let v = scalar(run_astro(
        "chandrasekhar_limit",
        vec![("mu_e", ParameterValue::Scalar(2.0))],
    ));
    let solar_mass = 1.989e30;
    assert!(
        v / solar_mass > 1.3 && v / solar_mass < 1.5,
        "≈1.4 M☉, got {}",
        v / solar_mass
    );
}

#[test]
fn distance_modulus() {
    let v = scalar(run_astro(
        "distance_modulus",
        vec![
            ("apparent_mag", ParameterValue::Scalar(5.0)),
            ("absolute_mag", ParameterValue::Scalar(5.0)),
        ],
    ));
    assert!((v - 10.0).abs() < 1e-6, "d = 10 pc when m = M, got {v}");
}

#[test]
fn main_sequence_lifetime() {
    let v = scalar(run_astro(
        "main_sequence_lifetime",
        vec![
            ("mass_solar", ParameterValue::Scalar(1.0)),
            ("luminosity_solar", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(
        v > 8e9 && v < 12e9,
        "Sun MS lifetime ≈ 10 Gyr, got {} Gyr",
        v / 1e9
    );
}

#[test]
fn stellar_flux_inverse_square_distance() {
    let l = 3.828e26;
    let f1 = scalar(run_astro(
        "stellar_flux",
        vec![
            ("luminosity", ParameterValue::Scalar(l)),
            ("distance", ParameterValue::Scalar(1.0e11)),
        ],
    ));
    let f2 = scalar(run_astro(
        "stellar_flux",
        vec![
            ("luminosity", ParameterValue::Scalar(l)),
            ("distance", ParameterValue::Scalar(2.0e11)),
        ],
    ));
    assert!((f1 / f2 - 4.0).abs() < 1e-10);
}

#[test]
fn mass_luminosity_monotonic_in_solar_range() {
    let l1 = scalar(run_astro(
        "mass_luminosity_relation",
        vec![("mass_solar", ParameterValue::Scalar(0.8))],
    ));
    let l2 = scalar(run_astro(
        "mass_luminosity_relation",
        vec![("mass_solar", ParameterValue::Scalar(1.0))],
    ));
    let l3 = scalar(run_astro(
        "mass_luminosity_relation",
        vec![("mass_solar", ParameterValue::Scalar(1.5))],
    ));
    assert!(l1 < l2 && l2 < l3);
}

#[test]
fn absolute_magnitude_and_distance_modulus_roundtrip() {
    let m = 9.2;
    let d = 50.0;
    let abs_mag = scalar(run_astro(
        "absolute_magnitude",
        vec![
            ("apparent_mag", ParameterValue::Scalar(m)),
            ("distance_pc", ParameterValue::Scalar(d)),
        ],
    ));
    let d_back = scalar(run_astro(
        "distance_modulus",
        vec![
            ("apparent_mag", ParameterValue::Scalar(m)),
            ("absolute_mag", ParameterValue::Scalar(abs_mag)),
        ],
    ));
    assert!((d_back - d).abs() / d < 1e-12);
}

#[test]
fn eddington_luminosity_scales_linearly_with_mass() {
    let l1 = scalar(run_astro(
        "eddington_luminosity",
        vec![("mass", ParameterValue::Scalar(1.0e30))],
    ));
    let l2 = scalar(run_astro(
        "eddington_luminosity",
        vec![("mass", ParameterValue::Scalar(2.0e30))],
    ));
    assert!((l2 / l1 - 2.0).abs() < 1e-12);
}
