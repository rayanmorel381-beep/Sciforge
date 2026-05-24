use sciforge_hub::prelude::*;

fn run_meteo(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Meteorology, name);
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
fn barometric_formula() {
    let v = scalar(run_meteo(
        "barometric_formula",
        vec![
            ("p0", ParameterValue::Scalar(101325.0)),
            ("m", ParameterValue::Scalar(0.029)),
            ("g", ParameterValue::Scalar(9.81)),
            ("h", ParameterValue::Scalar(0.0)),
            ("t", ParameterValue::Scalar(288.15)),
        ],
    ));
    assert!((v - 101325.0).abs() < 1.0, "P(0) = P0, got {v}");
}

#[test]
fn barometric_decreases_with_altitude() {
    let p0 = scalar(run_meteo(
        "barometric_formula",
        vec![
            ("p0", ParameterValue::Scalar(101325.0)),
            ("m", ParameterValue::Scalar(0.029)),
            ("g", ParameterValue::Scalar(9.81)),
            ("h", ParameterValue::Scalar(0.0)),
            ("t", ParameterValue::Scalar(288.15)),
        ],
    ));
    let p5 = scalar(run_meteo(
        "barometric_formula",
        vec![
            ("p0", ParameterValue::Scalar(101325.0)),
            ("m", ParameterValue::Scalar(0.029)),
            ("g", ParameterValue::Scalar(9.81)),
            ("h", ParameterValue::Scalar(5000.0)),
            ("t", ParameterValue::Scalar(288.15)),
        ],
    ));
    assert!(p5 < p0, "pressure should decrease with altitude");
    assert!(p5 > 50000.0, "5km pressure should be about half sea level");
}

#[test]
fn saturation_vapor_pressure() {
    let v = scalar(run_meteo(
        "saturation_vapor_pressure",
        vec![("t_celsius", ParameterValue::Scalar(100.0))],
    ));
    assert!(
        (v - 1013.25).abs() / 1013.25 < 0.05,
        "sat. VP at 100°C ≈ 1013 hPa, got {v}"
    );
}

#[test]
fn saturation_vapor_increases_with_temp() {
    let v20 = scalar(run_meteo(
        "saturation_vapor_pressure",
        vec![("t_celsius", ParameterValue::Scalar(20.0))],
    ));
    let v30 = scalar(run_meteo(
        "saturation_vapor_pressure",
        vec![("t_celsius", ParameterValue::Scalar(30.0))],
    ));
    assert!(v30 > v20, "SVP should increase with temperature");
}

#[test]
fn potential_temperature() {
    let v = scalar(run_meteo(
        "potential_temperature",
        vec![
            ("t", ParameterValue::Scalar(300.0)),
            ("p", ParameterValue::Scalar(1000.0)),
            ("p0", ParameterValue::Scalar(1000.0)),
        ],
    ));
    assert!(
        (v - 300.0).abs() < 1.0,
        "θ = T at 1000 hPa reference, got {v}"
    );
}

#[test]
fn scale_height() {
    let v = scalar(run_meteo(
        "scale_height",
        vec![
            ("t", ParameterValue::Scalar(288.15)),
            ("m", ParameterValue::Scalar(0.029)),
            ("g", ParameterValue::Scalar(9.81)),
        ],
    ));
    assert!(v > 7500.0 && v < 9000.0, "H ≈ 8.5 km for Earth, got {v}");
}

#[test]
fn lapse_rate_dry() {
    let v = scalar(run_meteo("lapse_rate_dry", vec![]));
    assert!((v - 9.8).abs() < 0.5, "dry lapse rate ≈ 9.8 K/km, got {v}");
}
