use sciforge_hub::prelude::*;

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
fn capacitance_parallel_plate() {
    let v = scalar(run_phys(
        "capacitance_parallel_plate",
        vec![
            ("area", ParameterValue::Scalar(1.0)),
            ("distance", ParameterValue::Scalar(0.001)),
            ("epsilon_r", ParameterValue::Scalar(1.0)),
        ],
    ));
    let expected = 8.854_187_812_8e-12 * 1.0 / 0.001;
    assert!((v - expected).abs() / expected < 1e-6);
}

#[test]
fn electric_field_point_charge() {
    let out = run_phys(
        "electric_field_point_charge",
        vec![
            ("q", ParameterValue::Scalar(1e-6)),
            ("r", ParameterValue::Vector(vec![1.0, 0.0, 0.0])),
        ],
    );
    match out {
        RunOutput::Triple(ex, ey, ez) => {
            let mag = (ex * ex + ey * ey + ez * ez).sqrt();
            let expected = 8.987_551_787e9 * 1e-6 / 1.0;
            assert!(
                (mag - expected).abs() / expected < 1e-4,
                "E = kQ/r², got {mag}"
            );
        }
        _ => panic!("expected Triple, got {out:?}"),
    }
}

#[test]
fn magnetic_field_wire() {
    let v = scalar(run_phys(
        "magnetic_field_wire",
        vec![
            ("current", ParameterValue::Scalar(10.0)),
            ("r", ParameterValue::Scalar(0.1)),
        ],
    ));
    let expected = 1.256_637_062_12e-6 * 10.0 / (2.0 * std::f64::consts::PI * 0.1);
    assert!((v - expected).abs() / expected < 1e-4);
}

#[test]
fn ohm_current() {
    let v = scalar(run_phys(
        "ohm_current",
        vec![
            ("v", ParameterValue::Scalar(12.0)),
            ("r", ParameterValue::Scalar(4.0)),
        ],
    ));
    assert!((v - 3.0).abs() < 1e-12, "I = V/R = 3A, got {v}");
}

#[test]
fn ohm_voltage() {
    let v = scalar(run_phys(
        "ohm_voltage",
        vec![
            ("i", ParameterValue::Scalar(2.5)),
            ("r", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!((v - 250.0).abs() < 1e-10);
}

#[test]
fn ohm_resistance() {
    let v = scalar(run_phys(
        "ohm_resistance",
        vec![
            ("v", ParameterValue::Scalar(220.0)),
            ("i", ParameterValue::Scalar(11.0)),
        ],
    ));
    assert!((v - 20.0).abs() < 1e-10);
}

#[test]
fn inductance_solenoid() {
    let v = scalar(run_phys(
        "inductance_solenoid",
        vec![
            ("n_turns", ParameterValue::Scalar(100.0)),
            ("area", ParameterValue::Scalar(0.01)),
            ("length", ParameterValue::Scalar(0.5)),
        ],
    ));
    assert!(v > 0.0, "inductance should be positive");
}

#[test]
fn malus_law() {
    let v = scalar(run_phys(
        "malus_law",
        vec![
            ("i0", ParameterValue::Scalar(100.0)),
            ("theta", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 100.0).abs() < 1e-10, "I = I0·cos²(0) = I0");
}

#[test]
fn ohm_law_roundtrip_consistency() {
    let v0 = 48.0;
    let r0 = 12.0;
    let i = scalar(run_phys(
        "ohm_current",
        vec![
            ("v", ParameterValue::Scalar(v0)),
            ("r", ParameterValue::Scalar(r0)),
        ],
    ));
    let v1 = scalar(run_phys(
        "ohm_voltage",
        vec![
            ("i", ParameterValue::Scalar(i)),
            ("r", ParameterValue::Scalar(r0)),
        ],
    ));
    let r1 = scalar(run_phys(
        "ohm_resistance",
        vec![
            ("v", ParameterValue::Scalar(v0)),
            ("i", ParameterValue::Scalar(i)),
        ],
    ));
    assert!((v1 - v0).abs() < 1e-12);
    assert!((r1 - r0).abs() < 1e-12);
}

#[test]
fn malus_law_quarter_turn_extinction() {
    let v = scalar(run_phys(
        "malus_law",
        vec![
            ("i0", ParameterValue::Scalar(100.0)),
            ("theta", ParameterValue::Scalar(std::f64::consts::FRAC_PI_2)),
        ],
    ));
    assert!(v.abs() < 1e-10);
}
