use sciforge_hub::prelude::*;

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
fn telomere_shortening() {
    let v = scalar(run_bio(
        "telomere_shortening",
        vec![
            ("initial_length", ParameterValue::Scalar(10000.0)),
            ("loss_per_division", ParameterValue::Scalar(50.0)),
            ("divisions", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!((v - 5000.0).abs() < 1e-6, "10000 - 50*100 = 5000, got {v}");
}

#[test]
fn gompertz_mortality_rate() {
    let v = scalar(run_bio(
        "gompertz_mortality_rate",
        vec![
            ("a", ParameterValue::Scalar(0.0001)),
            ("b", ParameterValue::Scalar(0.085)),
            ("age", ParameterValue::Scalar(70.0)),
        ],
    ));
    assert!(v > 0.0, "mortality rate should be positive");
    let v2 = scalar(run_bio(
        "gompertz_mortality_rate",
        vec![
            ("a", ParameterValue::Scalar(0.0001)),
            ("b", ParameterValue::Scalar(0.085)),
            ("age", ParameterValue::Scalar(80.0)),
        ],
    ));
    assert!(v2 > v, "mortality rate should increase with age");
}

#[test]
fn metabolic_rate_kleiber() {
    let v = scalar(run_bio(
        "metabolic_rate_kleiber",
        vec![("mass", ParameterValue::Scalar(70.0))],
    ));
    assert!(v > 0.0, "BMR should be positive for 70kg");
}

#[test]
fn codon_frequency() {
    let out = run_bio(
        "codon_frequency",
        vec![(
            "seq",
            ParameterValue::Text("ATGATGATGATGATGATGATGATGATGATG".to_string()),
        )],
    );
    match out {
        RunOutput::PairVec(v) => {
            assert!(!v.is_empty(), "should return codon frequencies");
        }
        _ => panic!("expected PairVec, got {out:?}"),
    }
}

#[test]
fn soil_respiration() {
    let v = scalar(run_bio(
        "soil_respiration",
        vec![
            ("temperature", ParameterValue::Scalar(25.0)),
            ("moisture", ParameterValue::Scalar(0.5)),
            ("q10", ParameterValue::Scalar(2.0)),
            ("r_ref", ParameterValue::Scalar(5.0)),
        ],
    ));
    assert!(v > 0.0, "soil respiration should be positive");
}

#[test]
fn dispatch_unknown_function_returns_error() {
    let exp = Experiment::new(DomainType::Biology, "unknown_biology_function");
    let result = ExperimentRunner::new().run(&exp);
    assert!(result.is_err());
}

#[test]
fn dispatch_missing_parameter_returns_error() {
    let exp = Experiment::new(DomainType::Biology, "telomere_shortening")
        .param("initial_length", ParameterValue::Scalar(10000.0));
    let result = ExperimentRunner::new().run(&exp);
    assert!(result.is_err());
}
