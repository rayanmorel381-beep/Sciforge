use sciforge::hub::prelude::*;

fn run_phys(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Physics, name);
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
fn carnot_efficiency() {
    let v = scalar(run_phys(
        "carnot_efficiency",
        vec![
            ("t_hot", ParameterValue::Scalar(600.0)),
            ("t_cold", ParameterValue::Scalar(300.0)),
        ],
    ));
    assert!(
        (v - 0.5).abs() < 1e-10,
        "Carnot η = 1 - Tc/Th = 0.5, got {v}"
    );
}

#[test]
fn ideal_gas_pressure() {
    let v = scalar(run_phys(
        "ideal_gas_pressure",
        vec![
            ("n_moles", ParameterValue::Scalar(1.0)),
            ("temperature", ParameterValue::Scalar(273.15)),
            ("volume", ParameterValue::Scalar(0.022414)),
        ],
    ));
    let expected = 1.0 * 8.314_462_618 * 273.15 / 0.022414;
    assert!(
        (v - expected).abs() / expected < 1e-4,
        "PV=nRT: got {v}, expected {expected}"
    );
}

#[test]
fn ideal_gas_temperature() {
    let v = scalar(run_phys(
        "ideal_gas_temperature",
        vec![
            ("pressure", ParameterValue::Scalar(101325.0)),
            ("volume", ParameterValue::Scalar(0.022414)),
            ("n_moles", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((v - 273.15).abs() < 1.0, "T ≈ 273.15 K, got {v}");
}

#[test]
fn stefan_boltzmann_power() {
    let v = scalar(run_phys(
        "stefan_boltzmann_power",
        vec![
            ("area", ParameterValue::Scalar(1.0)),
            ("temperature", ParameterValue::Scalar(1000.0)),
        ],
    ));
    let expected = 5.670_374_419e-8 * 1000.0_f64.powi(4);
    assert!((v - expected).abs() / expected < 1e-6);
}

#[test]
fn wien_displacement() {
    let v = scalar(run_phys(
        "wien_displacement",
        vec![("temperature", ParameterValue::Scalar(5778.0))],
    ));
    assert!(v > 4e-7 && v < 6e-7, "Wien peak for Sun ≈ 500nm, got {v}");
}

#[test]
fn boltzmann_distribution() {
    let v = scalar(run_phys(
        "boltzmann_distribution",
        vec![
            ("energy", ParameterValue::Scalar(0.0)),
            ("temperature", ParameterValue::Scalar(300.0)),
        ],
    ));
    assert!((v - 1.0).abs() < 1e-10, "P(E=0) = exp(0) = 1, got {v}");
}
