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
fn beaufort_to_m_s() {
    let v = scalar(run_meteo(
        "beaufort_to_m_s",
        vec![("b", ParameterValue::Scalar(8.0))],
    ));
    assert!(
        v > 15.0 && v < 25.0,
        "Beaufort 8 (gale) ~17-20 m/s, got {v}"
    );
}

#[test]
fn beaufort_zero_is_calm() {
    let v = scalar(run_meteo(
        "beaufort_to_m_s",
        vec![("b", ParameterValue::Scalar(0.0))],
    ));
    assert!(v.abs() < 1.0, "Beaufort 0 should be near-calm, got {v}");
}

#[test]
fn beaufort_monotonic() {
    let mut prev = scalar(run_meteo(
        "beaufort_to_m_s",
        vec![("b", ParameterValue::Scalar(0.0))],
    ));
    for b in [2.0, 4.0, 6.0, 8.0, 10.0, 12.0] {
        let cur = scalar(run_meteo(
            "beaufort_to_m_s",
            vec![("b", ParameterValue::Scalar(b))],
        ));
        assert!(
            cur >= prev,
            "wind speed should increase with Beaufort: b={b}, prev={prev}, cur={cur}"
        );
        prev = cur;
    }
}

#[test]
fn wind_chill() {
    let v = scalar(run_meteo(
        "wind_chill",
        vec![
            ("t", ParameterValue::Scalar(0.0)),
            ("v", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!(
        v < 0.0,
        "wind chill at 0°C, 10m/s should be negative, got {v}"
    );
}

#[test]
fn wind_chill_decreases_with_wind() {
    let wc1 = scalar(run_meteo(
        "wind_chill",
        vec![
            ("t", ParameterValue::Scalar(-5.0)),
            ("v", ParameterValue::Scalar(5.0)),
        ],
    ));
    let wc2 = scalar(run_meteo(
        "wind_chill",
        vec![
            ("t", ParameterValue::Scalar(-5.0)),
            ("v", ParameterValue::Scalar(20.0)),
        ],
    ));
    assert!(
        wc2 < wc1,
        "more wind -> colder wind chill: wc1={wc1}, wc2={wc2}"
    );
}
