use sciforge_hub::prelude::*;

fn run(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Astrobiology, name);
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
fn habitable_zone_inner() {
    let v = scalar(run(
        "habitable_zone_inner",
        vec![("luminosity", ParameterValue::Scalar(3.828e26))],
    ));
    assert!(v > 0.5 && v < 1.5, "inner HZ ~0.95 AU for Sun, got {v}");
}

#[test]
fn habitable_zone_outer() {
    let v = scalar(run(
        "habitable_zone_outer",
        vec![("luminosity", ParameterValue::Scalar(3.828e26))],
    ));
    assert!(v > 1.0 && v < 2.5, "outer HZ ~1.4-1.7 AU for Sun, got {v}");
}

#[test]
fn drake_equation() {
    let v = scalar(run(
        "drake_equation",
        vec![
            ("rate_star_formation", ParameterValue::Scalar(1.0)),
            ("fraction_planets", ParameterValue::Scalar(0.2)),
            ("habitable_per_system", ParameterValue::Scalar(1.0)),
            ("fraction_life", ParameterValue::Scalar(1.0)),
            ("fraction_intelligence", ParameterValue::Scalar(0.01)),
            ("fraction_communication", ParameterValue::Scalar(0.01)),
            ("civilization_lifetime", ParameterValue::Scalar(10000.0)),
        ],
    ));
    assert!(v > 0.0, "Drake N should be positive, got {v}");
}

#[test]
fn surface_gravity() {
    let v = scalar(run(
        "surface_gravity",
        vec![
            ("mass", ParameterValue::Scalar(5.972e24)),
            ("radius", ParameterValue::Scalar(6.371e6)),
        ],
    ));
    assert!((v - 9.81).abs() < 0.1, "Earth g ≈ 9.81, got {v}");
}

#[test]
fn uv_surface_flux() {
    let v = scalar(run(
        "uv_surface_flux",
        vec![
            ("incident_flux", ParameterValue::Scalar(10.0)),
            ("optical_depth", ParameterValue::Scalar(3.0)),
        ],
    ));
    assert!(v > 0.0 && v < 10.0, "UV flux should be attenuated, got {v}");
}
