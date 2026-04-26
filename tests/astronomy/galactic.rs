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
fn galactic_rotation_velocity() {
    let v = scalar(run_astro(
        "galactic_rotation_velocity",
        vec![
            ("mass_enclosed", ParameterValue::Scalar(1e11 * 1.989e30)),
            ("r", ParameterValue::Scalar(2.5e20)),
        ],
    ));
    assert!(v > 1e4, "galactic rotation > 10 km/s, got {v}");
}

#[test]
fn galactic_escape_velocity() {
    let v = scalar(run_astro(
        "galactic_escape_velocity",
        vec![
            ("mass_enclosed", ParameterValue::Scalar(1e11 * 1.989e30)),
            ("r", ParameterValue::Scalar(2.5e20)),
        ],
    ));
    assert!(v > 1e4, "galactic escape velocity > 10 km/s, got {v}");
}

#[test]
fn tidal_radius() {
    let v = scalar(run_astro(
        "tidal_radius",
        vec![
            ("m_cluster", ParameterValue::Scalar(1e5 * 1.989e30)),
            ("r_galactic", ParameterValue::Scalar(2.5e20)),
        ],
    ));
    assert!(v > 0.0, "tidal radius should be positive, got {v}");
}
