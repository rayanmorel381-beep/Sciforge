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
fn fish_growth_von_bertalanffy() {
    let v = scalar(run_bio(
        "fish_growth_von_bertalanffy",
        vec![
            ("l_inf", ParameterValue::Scalar(100.0)),
            ("k", ParameterValue::Scalar(0.2)),
            ("t", ParameterValue::Scalar(0.0)),
            ("t0", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 0.0).abs() < 1e-6, "at t=t0, L=0, got {v}");
}

#[test]
fn maximum_sustainable_yield() {
    let v = scalar(run_bio(
        "maximum_sustainable_yield",
        vec![
            ("r", ParameterValue::Scalar(0.5)),
            ("k", ParameterValue::Scalar(1000.0)),
        ],
    ));
    let expected = 0.5 * 1000.0 / 4.0;
    assert!((v - expected).abs() < 1e-6, "MSY=rK/4=125, got {v}");
}

#[test]
fn ocean_acidification_ph() {
    let v = scalar(run_bio(
        "ocean_acidification_ph",
        vec![
            ("pco2", ParameterValue::Scalar(280.0)),
            ("alkalinity", ParameterValue::Scalar(2300.0)),
            ("temperature", ParameterValue::Scalar(25.0)),
        ],
    ));
    assert!(
        v > 0.0 && v.is_finite(),
        "pH should be finite and positive, got {v}"
    );
}

#[test]
fn bleaching_thermal_threshold() {
    let v = scalar(run_bio(
        "bleaching_thermal_threshold",
        vec![
            ("sst", ParameterValue::Scalar(31.5)),
            ("mmm", ParameterValue::Scalar(29.0)),
        ],
    ));
    assert!((v - 1.5).abs() < 1e-8, "(31.5-29-1)=1.5, got {v}");
}
