use sciforge::hub::prelude::*;

fn run_meteo(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Meteorology, name);
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
fn stefan_boltzmann_flux() {
    let v = scalar(run_meteo(
        "stefan_boltzmann_flux",
        vec![("t", ParameterValue::Scalar(288.0))],
    ));
    let expected = 5.670_374_419e-8 * 288.0_f64.powi(4);
    assert!((v - expected).abs() / expected < 1e-4);
}

#[test]
fn effective_temperature() {
    let v = scalar(run_meteo(
        "effective_temperature",
        vec![
            ("solar_flux", ParameterValue::Scalar(1361.0)),
            ("albedo", ParameterValue::Scalar(0.3)),
        ],
    ));
    assert!(v > 250.0 && v < 260.0, "Earth effective T ≈ 255 K, got {v}");
}

#[test]
fn albedo_reflected() {
    let v = scalar(run_meteo(
        "albedo_reflected",
        vec![
            ("solar_flux", ParameterValue::Scalar(1361.0)),
            ("albedo", ParameterValue::Scalar(0.3)),
        ],
    ));
    assert!(
        (v - 408.3).abs() < 1.0,
        "reflected = 1361·0.3 = 408.3, got {v}"
    );
}

#[test]
fn planck_function() {
    let v = scalar(run_meteo(
        "planck_function",
        vec![
            ("wavelength", ParameterValue::Scalar(500e-9)),
            ("t", ParameterValue::Scalar(5778.0)),
        ],
    ));
    assert!(v > 0.0, "Planck function should be positive");
}

#[test]
fn radiative_forcing_co2() {
    let v = scalar(run_meteo(
        "radiative_forcing_co2",
        vec![
            ("c", ParameterValue::Scalar(560.0)),
            ("c0", ParameterValue::Scalar(280.0)),
        ],
    ));
    assert!(v > 3.0 && v < 4.5, "2×CO2 forcing ≈ 3.7 W/m², got {v}");
}

#[test]
fn stefan_boltzmann_flux_monotonic_with_temperature_sweep() {
    let mut prev = scalar(run_meteo(
        "stefan_boltzmann_flux",
        vec![("t", ParameterValue::Scalar(200.0))],
    ));
    for t in [220.0, 240.0, 260.0, 280.0, 300.0, 320.0] {
        let cur = scalar(run_meteo(
            "stefan_boltzmann_flux",
            vec![("t", ParameterValue::Scalar(t))],
        ));
        assert!(
            cur > prev,
            "flux should increase with T: t={t}, prev={prev}, cur={cur}"
        );
        prev = cur;
    }
}

#[test]
fn radiative_forcing_co2_zero_when_equal_concentrations() {
    let rf = scalar(run_meteo(
        "radiative_forcing_co2",
        vec![
            ("c", ParameterValue::Scalar(420.0)),
            ("c0", ParameterValue::Scalar(420.0)),
        ],
    ));
    assert!(rf.abs() < 1e-12);
}
