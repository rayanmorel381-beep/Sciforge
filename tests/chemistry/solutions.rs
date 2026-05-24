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
fn boiling_point_elevation() {
    let v = scalar(run_chem(
        "boiling_point_elevation",
        vec![
            ("kb", ParameterValue::Scalar(0.512)),
            ("molality", ParameterValue::Scalar(1.0)),
            ("vant_hoff_factor", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((v - 0.512).abs() < 1e-6, "dTb=Kb*m=0.512, got {v}");
}

#[test]
fn freezing_point_depression() {
    let v = scalar(run_chem(
        "freezing_point_depression",
        vec![
            ("kf", ParameterValue::Scalar(1.86)),
            ("molality", ParameterValue::Scalar(1.0)),
            ("vant_hoff_factor", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((v - 1.86).abs() < 1e-6, "dTf=Kf*m=1.86, got {v}");
}

#[test]
fn osmotic_pressure() {
    let v = scalar(run_chem(
        "osmotic_pressure",
        vec![
            ("molarity", ParameterValue::Scalar(1.0)),
            ("temperature", ParameterValue::Scalar(298.15)),
            ("vant_hoff_factor", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(v > 0.0, "osmotic pressure should be positive, got {v}");
}

#[test]
fn raoult_law() {
    let v = scalar(run_chem(
        "raoult_law",
        vec![
            ("x_solvent", ParameterValue::Scalar(1.0)),
            ("p0_solvent", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!((v - 100.0).abs() < 1e-8, "pure solvent => P=P*, got {v}");
}
