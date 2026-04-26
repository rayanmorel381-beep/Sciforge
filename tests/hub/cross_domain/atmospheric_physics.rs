use sciforge::hub::prelude::*;

fn run(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::AtmosphericPhysics, name);
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
    let v = scalar(run(
        "stefan_boltzmann_flux",
        vec![
            ("temperature", ParameterValue::Scalar(5778.0)),
            ("emissivity", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(v > 6e7, "Sun surface flux ≈ 6.3e7 W/m², got {v}");
}

#[test]
fn wien_peak_wavelength() {
    let v = scalar(run(
        "wien_peak_wavelength",
        vec![("temperature", ParameterValue::Scalar(5778.0))],
    ));
    assert!((v - 5.02e-7).abs() < 1e-8, "Sun peak ≈ 502nm, got {v}");
}

#[test]
fn dry_adiabatic_lapse_rate() {
    let v = scalar(run(
        "dry_adiabatic_lapse_rate",
        vec![
            ("gravity", ParameterValue::Scalar(9.81)),
            ("specific_heat", ParameterValue::Scalar(1004.0)),
        ],
    ));
    assert!((v - 9.77e-3).abs() < 1e-4, "DALR ≈ 9.77 K/km, got {v}");
}

#[test]
fn pressure_at_altitude() {
    let v = scalar(run(
        "pressure_at_altitude",
        vec![
            ("surface_pressure", ParameterValue::Scalar(101325.0)),
            ("altitude", ParameterValue::Scalar(0.0)),
            ("scale_height", ParameterValue::Scalar(8500.0)),
        ],
    ));
    assert!((v - 101325.0).abs() < 1.0, "P at h=0 = P0, got {v}");
}
