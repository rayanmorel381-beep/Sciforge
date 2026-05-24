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
fn kepler_period_earth() {
    let v = scalar(run_astro(
        "kepler_period",
        vec![
            ("a", ParameterValue::Scalar(1.496e11)),
            ("mu", ParameterValue::Scalar(6.674e-11 * 1.989e30)),
        ],
    ));
    let year_s = 365.25 * 86400.0;
    assert!(
        (v - year_s).abs() / year_s < 0.01,
        "Earth period ≈ 1 year, got {v} s"
    );
}

#[test]
fn escape_velocity() {
    let v = scalar(run_astro(
        "escape_velocity",
        vec![
            ("mu", ParameterValue::Scalar(6.674e-11 * 5.972e24)),
            ("r", ParameterValue::Scalar(6.371e6)),
        ],
    ));
    assert!(
        v > 11000.0 && v < 11300.0,
        "Earth escape v ≈ 11.2 km/s, got {v} m/s"
    );
}

#[test]
fn circular_velocity() {
    let v = scalar(run_astro(
        "circular_velocity",
        vec![
            ("mu", ParameterValue::Scalar(6.674e-11 * 5.972e24)),
            ("r", ParameterValue::Scalar(6.371e6 + 400e3)),
        ],
    ));
    assert!(
        v > 7500.0 && v < 8000.0,
        "ISS orbit ≈ 7.66 km/s, got {v} m/s"
    );
}

#[test]
fn vis_viva_circular() {
    let r = 6.371e6 + 400e3;
    let v = scalar(run_astro(
        "vis_viva",
        vec![
            ("mu", ParameterValue::Scalar(6.674e-11 * 5.972e24)),
            ("r", ParameterValue::Scalar(r)),
            ("a", ParameterValue::Scalar(r)),
        ],
    ));
    let v_circ = scalar(run_astro(
        "circular_velocity",
        vec![
            ("mu", ParameterValue::Scalar(6.674e-11 * 5.972e24)),
            ("r", ParameterValue::Scalar(r)),
        ],
    ));
    assert!(
        (v.sqrt() - v_circ).abs() / v_circ < 1e-6,
        "vis-viva circular case"
    );
}

#[test]
fn roche_limit() {
    let v = scalar(run_astro(
        "roche_limit",
        vec![
            ("r_primary", ParameterValue::Scalar(6.371e6)),
            ("rho_primary", ParameterValue::Scalar(5515.0)),
            ("rho_secondary", ParameterValue::Scalar(3340.0)),
        ],
    ));
    assert!(v > 9e6 && v < 20e6, "Earth-Moon Roche ≈ 9500 km, got {v} m");
}

#[test]
fn sphere_of_influence_earth() {
    let v = scalar(run_astro(
        "sphere_of_influence",
        vec![
            ("a", ParameterValue::Scalar(1.496e11)),
            ("m_planet", ParameterValue::Scalar(5.972e24)),
            ("m_star", ParameterValue::Scalar(1.989e30)),
        ],
    ));
    assert!(v > 8e8 && v < 1e9, "Earth SOI ≈ 925,000 km, got {v} m");
}

#[test]
fn escape_vs_circular_velocity_ratio() {
    let mu = 6.674e-11 * 5.972e24;
    let r = 6.371e6;
    let v_esc = scalar(run_astro(
        "escape_velocity",
        vec![
            ("mu", ParameterValue::Scalar(mu)),
            ("r", ParameterValue::Scalar(r)),
        ],
    ));
    let v_circ = scalar(run_astro(
        "circular_velocity",
        vec![
            ("mu", ParameterValue::Scalar(mu)),
            ("r", ParameterValue::Scalar(r)),
        ],
    ));
    assert!((v_esc / v_circ - 2.0_f64.sqrt()).abs() < 1e-10);
}

#[test]
fn periapsis_apoapsis_consistency() {
    let a = 12000.0;
    let e = 0.2;
    let rp = scalar(run_astro(
        "periapsis",
        vec![
            ("a", ParameterValue::Scalar(a)),
            ("e", ParameterValue::Scalar(e)),
        ],
    ));
    let ra = scalar(run_astro(
        "apoapsis",
        vec![
            ("a", ParameterValue::Scalar(a)),
            ("e", ParameterValue::Scalar(e)),
        ],
    ));
    assert!((rp + ra - 2.0 * a).abs() < 1e-10);
}

#[test]
fn true_anomaly_radius_matches_extrema() {
    let a = 10000.0;
    let e = 0.3;
    let r0 = scalar(run_astro(
        "true_anomaly_to_radius",
        vec![
            ("a", ParameterValue::Scalar(a)),
            ("e", ParameterValue::Scalar(e)),
            ("theta", ParameterValue::Scalar(0.0)),
        ],
    ));
    let rpi = scalar(run_astro(
        "true_anomaly_to_radius",
        vec![
            ("a", ParameterValue::Scalar(a)),
            ("e", ParameterValue::Scalar(e)),
            ("theta", ParameterValue::Scalar(std::f64::consts::PI)),
        ],
    ));
    let rp = scalar(run_astro(
        "periapsis",
        vec![
            ("a", ParameterValue::Scalar(a)),
            ("e", ParameterValue::Scalar(e)),
        ],
    ));
    let ra = scalar(run_astro(
        "apoapsis",
        vec![
            ("a", ParameterValue::Scalar(a)),
            ("e", ParameterValue::Scalar(e)),
        ],
    ));
    assert!((r0 - rp).abs() < 1e-10);
    assert!((rpi - ra).abs() < 1e-10);
}

#[test]
fn hohmann_delta_v_positive_and_zero_identity() {
    let mu = 3.986004418e14;
    let r1 = 7000e3;
    let r2 = 42164e3;
    let dv = scalar(run_astro(
        "hohmann_delta_v",
        vec![
            ("mu", ParameterValue::Scalar(mu)),
            ("r1", ParameterValue::Scalar(r1)),
            ("r2", ParameterValue::Scalar(r2)),
        ],
    ));
    let dv_zero = scalar(run_astro(
        "hohmann_delta_v",
        vec![
            ("mu", ParameterValue::Scalar(mu)),
            ("r1", ParameterValue::Scalar(r1)),
            ("r2", ParameterValue::Scalar(r1)),
        ],
    ));
    assert!(dv > 0.0);
    assert!(dv_zero.abs() < 1e-12);
}
