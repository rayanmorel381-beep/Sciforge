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
fn ideal_gas_pressure_chem() {
    let v = scalar(run_chem(
        "ideal_gas_pressure",
        vec![
            ("n", ParameterValue::Scalar(1.0)),
            ("t", ParameterValue::Scalar(273.15)),
            ("v", ParameterValue::Scalar(0.022414)),
        ],
    ));
    assert!((v - 101325.0).abs() / 101325.0 < 0.01, "≈1 atm, got {v}");
}

#[test]
fn boyles_law() {
    let v = scalar(run_chem(
        "boyles_law",
        vec![
            ("p1", ParameterValue::Scalar(2.0)),
            ("v1", ParameterValue::Scalar(3.0)),
            ("v2", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((v - 6.0).abs() < 1e-10, "P2 = P1*V1/V2 = 6.0, got {v}");
}

#[test]
fn charles_law() {
    let v = scalar(run_chem(
        "charles_law",
        vec![
            ("v1", ParameterValue::Scalar(1.0)),
            ("t1", ParameterValue::Scalar(300.0)),
            ("t2", ParameterValue::Scalar(600.0)),
        ],
    ));
    assert!((v - 2.0).abs() < 1e-10, "double T → double V, got {v}");
}

#[test]
fn dalton_partial_pressure() {
    let v = scalar(run_chem(
        "dalton_partial_pressure",
        vec![
            ("mole_fraction", ParameterValue::Scalar(0.21)),
            ("total_pressure", ParameterValue::Scalar(101325.0)),
        ],
    ));
    let expected = 0.21 * 101325.0;
    assert!((v - expected).abs() < 1.0, "Pi = xi·P, got {v}");
}

#[test]
fn grahams_law_effusion() {
    let v = scalar(run_chem(
        "grahams_law_effusion",
        vec![
            ("m1", ParameterValue::Scalar(2.0)),
            ("m2", ParameterValue::Scalar(32.0)),
        ],
    ));
    assert!((v - 4.0).abs() < 1e-10, "r1/r2 = √(M2/M1) = 4, got {v}");
}

#[test]
fn van_der_waals_pressure() {
    let v = scalar(run_chem(
        "van_der_waals_pressure",
        vec![
            ("n", ParameterValue::Scalar(1.0)),
            ("v", ParameterValue::Scalar(0.022414)),
            ("t", ParameterValue::Scalar(273.15)),
            ("a", ParameterValue::Scalar(0.0)),
            ("b", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!(
        (v - 101325.0).abs() / 101325.0 < 0.01,
        "vdW with a=b=0 ≈ ideal gas"
    );
}

#[test]
fn ideal_gas_inverse_temperature_roundtrip() {
    let p = scalar(run_chem(
        "ideal_gas_pressure",
        vec![
            ("n", ParameterValue::Scalar(2.0)),
            ("t", ParameterValue::Scalar(350.0)),
            ("v", ParameterValue::Scalar(0.1)),
        ],
    ));
    let t_back = scalar(run_chem(
        "ideal_gas_temperature",
        vec![
            ("p", ParameterValue::Scalar(p)),
            ("n", ParameterValue::Scalar(2.0)),
            ("v", ParameterValue::Scalar(0.1)),
        ],
    ));
    assert!((t_back - 350.0).abs() / 350.0 < 1e-12);
}

#[test]
fn boyles_law_pressure_halves_when_volume_doubles() {
    let p2 = scalar(run_chem(
        "boyles_law",
        vec![
            ("p1", ParameterValue::Scalar(8.0)),
            ("v1", ParameterValue::Scalar(1.0)),
            ("v2", ParameterValue::Scalar(2.0)),
        ],
    ));
    assert!((p2 - 4.0).abs() < 1e-12);
}
