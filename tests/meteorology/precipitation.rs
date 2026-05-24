use sciforge_hub::prelude::*;

fn run_meteo(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Meteorology, name);
    for (k, v) in params {
        exp = exp.param(k, v);
    }
    ExperimentRunner::new()
        .run(&exp)
        .unwrap_or_else(|_| panic!("dispatch \'{name}\' failed"))
}

fn scalar(o: RunOutput) -> f64 {
    match o {
        RunOutput::Scalar(v) => v,
        _ => panic!("expected Scalar, got {o:?}"),
    }
}

#[test]
fn rain_rate_marshall_palmer() {
    let v = scalar(run_meteo(
        "rain_rate_marshall_palmer",
        vec![("z", ParameterValue::Scalar(40.0))],
    ));
    assert!(v > 0.0, "rain rate should be positive, got {v}");
}

#[test]
fn radar_reflectivity() {
    let v = scalar(run_meteo(
        "radar_reflectivity",
        vec![("rain_rate", ParameterValue::Scalar(5.0))],
    ));
    assert!(v > 0.0, "reflectivity should be positive for rain, got {v}");
}

#[test]
fn terminal_velocity_raindrop() {
    let v = scalar(run_meteo(
        "terminal_velocity_raindrop",
        vec![("diameter_mm", ParameterValue::Scalar(3.0))],
    ));
    assert!(
        v > 5.0 && v < 12.0,
        "terminal velocity ~8 m/s for 3mm drop, got {v}"
    );
}

#[test]
fn thornthwaite_pet() {
    let v = scalar(run_meteo(
        "thornthwaite_pet",
        vec![
            ("t_mean", ParameterValue::Scalar(15.0)),
            ("heat_index", ParameterValue::Scalar(60.0)),
            ("day_length_hours", ParameterValue::Scalar(12.0)),
        ],
    ));
    assert!(v > 0.0, "PET should be positive, got {v}");
}

#[test]
fn penman_evaporation() {
    let v = scalar(run_meteo(
        "penman_evaporation",
        vec![
            ("delta", ParameterValue::Scalar(0.1)),
            ("rn", ParameterValue::Scalar(200.0)),
            ("gamma", ParameterValue::Scalar(0.067)),
            ("ea", ParameterValue::Scalar(1.5)),
            ("u", ParameterValue::Scalar(2.0)),
        ],
    ));
    assert!(v > 0.0, "evaporation should be positive, got {v}");
}

#[test]
fn intensity_duration_frequency() {
    let v = scalar(run_meteo(
        "intensity_duration_frequency",
        vec![
            ("a", ParameterValue::Scalar(50.0)),
            ("b", ParameterValue::Scalar(10.0)),
            ("duration", ParameterValue::Scalar(60.0)),
            ("return_period", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!(v > 0.0, "IDF intensity should be positive, got {v}");
}

#[test]
fn scs_curve_number_runoff() {
    let v = scalar(run_meteo(
        "scs_curve_number_runoff",
        vec![
            ("p", ParameterValue::Scalar(50.0)),
            ("cn", ParameterValue::Scalar(70.0)),
        ],
    ));
    assert!(
        (0.0..=50.0).contains(&v),
        "runoff <= precipitation, got {v}"
    );
}

#[test]
fn antecedent_precipitation_index() {
    let v = scalar(run_meteo(
        "antecedent_precipitation_index",
        vec![
            ("prev_api", ParameterValue::Scalar(10.0)),
            ("rainfall", ParameterValue::Scalar(5.0)),
            ("k", ParameterValue::Scalar(0.85)),
        ],
    ));
    let expected = 0.85 * 10.0 + 5.0;
    assert!(
        (v - expected).abs() < 0.1,
        "API = k*prev + R = {expected}, got {v}"
    );
}
