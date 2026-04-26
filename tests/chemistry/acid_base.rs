use sciforge::hub::prelude::*;

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
fn henderson_hasselbalch() {
    let v = scalar(run_chem(
        "henderson_hasselbalch",
        vec![
            ("pka", ParameterValue::Scalar(4.75)),
            ("acid", ParameterValue::Scalar(0.1)),
            ("base", ParameterValue::Scalar(0.1)),
        ],
    ));
    assert!((v - 4.75).abs() < 1e-10, "pH = pKa when [A-]=[HA], got {v}");
}

#[test]
fn henderson_hasselbalch_excess_base() {
    let v = scalar(run_chem(
        "henderson_hasselbalch",
        vec![
            ("pka", ParameterValue::Scalar(4.75)),
            ("acid", ParameterValue::Scalar(0.01)),
            ("base", ParameterValue::Scalar(0.1)),
        ],
    ));
    assert!(v > 4.75, "pH > pKa when [A-] > [HA], got {v}");
}

#[test]
fn ph_strong_acid() {
    let v = scalar(run_chem(
        "ph_strong_acid",
        vec![("concentration", ParameterValue::Scalar(0.01))],
    ));
    assert!((v - 2.0).abs() < 1e-10, "pH = -log10(0.01) = 2, got {v}");
}

#[test]
fn ph_strong_base() {
    let v = scalar(run_chem(
        "ph_strong_base",
        vec![("concentration", ParameterValue::Scalar(0.01))],
    ));
    assert!((v - 12.0).abs() < 1e-6, "pH ≈ 12 for 0.01M NaOH, got {v}");
}

#[test]
fn pka_roundtrip() {
    let pka = 4.75;
    let ka = scalar(run_chem(
        "ka_from_pka",
        vec![("pka", ParameterValue::Scalar(pka))],
    ));
    let pka_back = scalar(run_chem(
        "pka_from_ka",
        vec![("ka", ParameterValue::Scalar(ka))],
    ));
    assert!(
        (pka_back - pka).abs() < 1e-10,
        "pKa roundtrip failed: {pka} -> {ka} -> {pka_back}"
    );
}

#[test]
fn equivalence_point_volume() {
    let v = scalar(run_chem(
        "equivalence_point_volume",
        vec![
            ("c_analyte", ParameterValue::Scalar(0.1)),
            ("v_analyte", ParameterValue::Scalar(0.025)),
            ("c_titrant", ParameterValue::Scalar(0.1)),
        ],
    ));
    assert!(
        (v - 0.025).abs() < 1e-10,
        "equal concentrations → equal volumes"
    );
}
