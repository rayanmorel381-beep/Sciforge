use sciforge::hub::prelude::*;

fn run_astro(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Astronomy, name);
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
fn surface_gravity() {
    let v = scalar(run_astro(
        "surface_gravity",
        vec![
            ("m", ParameterValue::Scalar(5.972e24)),
            ("r", ParameterValue::Scalar(6.371e6)),
        ],
    ));
    assert!((v - 9.81).abs() < 0.1, "g_Earth≈9.81, got {v}");
}

#[test]
fn habitable_zone_inner() {
    let v = scalar(run_astro(
        "habitable_zone_inner",
        vec![("luminosity_solar", ParameterValue::Scalar(1.0))],
    ));
    assert!(v > 0.5 && v < 1.5, "inner HZ for Sun ≈0.95 AU, got {v}");
}

#[test]
fn habitable_zone_outer() {
    let v = scalar(run_astro(
        "habitable_zone_outer",
        vec![("luminosity_solar", ParameterValue::Scalar(1.0))],
    ));
    assert!(v > 1.0 && v < 2.5, "outer HZ for Sun ≈1.7 AU, got {v}");
}
