use sciforge::hub::prelude::*;

fn run_bio(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Biology, name);
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
fn glomerular_filtration_rate() {
    let v = scalar(run_bio(
        "glomerular_filtration_rate",
        vec![
            ("kf", ParameterValue::Scalar(12.5)),
            ("p_gc", ParameterValue::Scalar(60.0)),
            ("p_bs", ParameterValue::Scalar(18.0)),
            ("pi_gc", ParameterValue::Scalar(32.0)),
        ],
    ));
    assert!(v > 0.0 && v < 300.0, "GFR in reasonable range, got {v}");
}

#[test]
fn alveolar_gas_equation() {
    let v = scalar(run_bio(
        "alveolar_gas_equation",
        vec![
            ("fio2", ParameterValue::Scalar(0.21)),
            ("p_atm", ParameterValue::Scalar(760.0)),
            ("p_h2o", ParameterValue::Scalar(47.0)),
            ("paco2", ParameterValue::Scalar(40.0)),
            ("rq", ParameterValue::Scalar(0.8)),
        ],
    ));
    assert!(v > 80.0 && v < 120.0, "PAO2 should be ~100 mmHg, got {v}");
}

#[test]
fn oxygen_delivery() {
    let v = scalar(run_bio(
        "oxygen_delivery",
        vec![
            ("cardiac_output", ParameterValue::Scalar(5.0)),
            ("cao2", ParameterValue::Scalar(200.0)),
        ],
    ));
    assert!(v > 0.0, "DO2 should be positive, got {v}");
}

#[test]
fn frank_starling_mechanism() {
    let v = scalar(run_bio(
        "frank_starling_mechanism",
        vec![
            ("end_diastolic_volume", ParameterValue::Scalar(120.0)),
            ("contractility", ParameterValue::Scalar(1.0)),
            ("max_stroke_volume", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!(v > 0.0, "stroke volume should be positive, got {v}");
}
