use sciforge_hub::prelude::*;

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
fn julian_date_to_j2000_century() {
    let v = scalar(run_astro(
        "julian_date_to_j2000_century",
        vec![("jd", ParameterValue::Scalar(2451545.0))],
    ));
    assert!(v.abs() < 1e-8, "J2000.0 => T=0, got {v}");
}

#[test]
fn gravitational_force() {
    let v = scalar(run_astro(
        "gravitational_force",
        vec![
            ("m1", ParameterValue::Scalar(5.972e24)),
            ("m2", ParameterValue::Scalar(7.342e22)),
            ("r", ParameterValue::Scalar(3.844e8)),
        ],
    ));
    assert!(v > 1e19, "Earth-Moon force > 1e19 N, got {v}");
}

#[test]
fn synodic_period() {
    let v = scalar(run_astro(
        "synodic_period",
        vec![
            ("p1", ParameterValue::Scalar(365.25)),
            ("p2", ParameterValue::Scalar(687.0)),
        ],
    ));
    assert!(
        (v - 779.9).abs() < 5.0,
        "synodic Earth-Mars ≈780 d, got {v}"
    );
}

#[test]
fn hill_sphere() {
    let v = scalar(run_astro(
        "hill_sphere",
        vec![
            ("a", ParameterValue::Scalar(1.496e11)),
            ("m", ParameterValue::Scalar(5.972e24)),
            ("m_star", ParameterValue::Scalar(1.989e30)),
            ("e", ParameterValue::Scalar(0.0167)),
        ],
    ));
    assert!(v > 1e9, "Earth Hill sphere > 1e9 m, got {v}");
}
