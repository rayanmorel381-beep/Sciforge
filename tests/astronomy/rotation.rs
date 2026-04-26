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
fn surface_velocity_at_latitude() {
    let omega = 2.0 * std::f64::consts::PI / 86400.0;
    let v = scalar(run_astro(
        "surface_velocity_at_latitude",
        vec![
            ("omega", ParameterValue::Scalar(omega)),
            ("r", ParameterValue::Scalar(6.371e6)),
            ("phi", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!(
        (v - 463.0).abs() < 5.0,
        "equatorial velocity ≈463 m/s, got {v}"
    );
}

#[test]
fn rotational_kinetic_energy() {
    let omega = 2.0 * std::f64::consts::PI / 86400.0;
    let inertia = 0.3308 * 5.972e24 * (6.371e6_f64).powi(2);
    let v = scalar(run_astro(
        "rotational_kinetic_energy",
        vec![
            ("inertia", ParameterValue::Scalar(inertia)),
            ("omega", ParameterValue::Scalar(omega)),
        ],
    ));
    assert!(v > 1e28, "Earth rotational KE > 1e28 J, got {v}");
}
