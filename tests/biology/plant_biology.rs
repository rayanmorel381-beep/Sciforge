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
fn transpiration_rate() {
    let v = scalar(run_bio(
        "transpiration_rate",
        vec![
            ("stomatal_conductance", ParameterValue::Scalar(0.3)),
            ("vpd", ParameterValue::Scalar(1.5)),
        ],
    ));
    assert!(v > 0.0, "transpiration should be positive, got {v}");
}

#[test]
fn water_potential() {
    let v = scalar(run_bio(
        "water_potential",
        vec![
            ("osmotic", ParameterValue::Scalar(-1.0)),
            ("pressure", ParameterValue::Scalar(0.5)),
            ("gravitational", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - (-0.5)).abs() < 1e-8, "psi=Ys+Yp+Yg=-0.5, got {v}");
}

#[test]
fn beer_lambert_canopy() {
    let v = scalar(run_bio(
        "beer_lambert_canopy",
        vec![
            ("light_above", ParameterValue::Scalar(1000.0)),
            ("k", ParameterValue::Scalar(0.5)),
            ("lai", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!(
        (v - 1000.0).abs() < 1e-6,
        "LAI=0 => no attenuation, got {v}"
    );
}

#[test]
fn photosynthesis_light_response() {
    let v = scalar(run_bio(
        "plant_light_response_curve",
        vec![
            ("phi", ParameterValue::Scalar(0.06)),
            ("ppfd", ParameterValue::Scalar(500.0)),
            ("amax", ParameterValue::Scalar(20.0)),
            ("theta", ParameterValue::Scalar(0.7)),
            ("rd", ParameterValue::Scalar(2.0)),
        ],
    ));
    assert!(
        v > 0.0,
        "net photosynthesis positive at reasonable light, got {v}"
    );
}
