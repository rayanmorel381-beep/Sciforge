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
fn ohm_current() {
    let v = scalar(run_phys(
        "ohm_current",
        vec![
            ("v", ParameterValue::Scalar(12.0)),
            ("r", ParameterValue::Scalar(4.0)),
        ],
    ));
    assert!((v - 3.0).abs() < 1e-8, "I=V/R=12/4=3, got {v}");
}

#[test]
fn voltage_divider() {
    let v = scalar(run_phys(
        "electronics::circuits::voltage_divider",
        vec![
            ("v_in", ParameterValue::Scalar(10.0)),
            ("r1", ParameterValue::Scalar(1000.0)),
            ("r2", ParameterValue::Scalar(1000.0)),
        ],
    ));
    assert!((v - 5.0).abs() < 1e-8, "Vout=Vin*R2/(R1+R2)=5, got {v}");
}

#[test]
fn rlc_resonant_frequency() {
    let v = scalar(run_phys(
        "rlc_resonant_frequency",
        vec![
            ("l", ParameterValue::Scalar(1e-3)),
            ("c", ParameterValue::Scalar(1e-6)),
        ],
    ));
    assert!(v > 0.0, "resonant frequency should be positive, got {v}");
}

#[test]
fn diode_shockley() {
    let v = scalar(run_phys(
        "diode_shockley",
        vec![
            ("is", ParameterValue::Scalar(1e-12)),
            ("v", ParameterValue::Scalar(0.0)),
            ("n", ParameterValue::Scalar(1.0)),
            ("vt", ParameterValue::Scalar(0.02585)),
        ],
    ));
    assert!(v.abs() < 1e-10, "at V=0, I≈0, got {v}");
}
