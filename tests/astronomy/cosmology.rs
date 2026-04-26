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

fn assert_close_rel(actual: f64, expected: f64, rtol: f64, context: &str) {
    let scale = expected.abs().max(1.0);
    let rel = (actual - expected).abs() / scale;
    assert!(
        rel <= rtol,
        "{context}: actual={actual}, expected={expected}, rel={rel}, rtol={rtol}"
    );
}

#[test]
fn hubble_velocity() {
    let v = scalar(run_astro(
        "hubble_velocity",
        vec![
            ("distance", ParameterValue::Scalar(100.0)),
            ("h0", ParameterValue::Scalar(70.0)),
        ],
    ));
    assert!((v - 7000.0).abs() < 1e-6, "v = H0·d = 7000 km/s, got {v}");
}

#[test]
fn critical_density() {
    let v = scalar(run_astro(
        "critical_density_from_h0",
        vec![("h0", ParameterValue::Scalar(67.4))],
    ));
    assert!(v > 8e-27 && v < 1e-26, "ρ_c ≈ 9.5e-27 kg/m³, got {v}");
}

#[test]
fn scale_factor() {
    let v = scalar(run_astro(
        "scale_factor",
        vec![("redshift", ParameterValue::Scalar(0.0))],
    ));
    assert!((v - 1.0).abs() < 1e-12, "a(z=0) = 1, got {v}");
}

#[test]
fn scale_factor_high_z() {
    let v = scalar(run_astro(
        "scale_factor",
        vec![("redshift", ParameterValue::Scalar(1.0))],
    ));
    assert!((v - 0.5).abs() < 1e-12, "a(z=1) = 0.5, got {v}");
}

#[test]
fn cmb_temperature_at_z() {
    let v = scalar(run_astro(
        "cmb_temperature",
        vec![
            ("t0", ParameterValue::Scalar(2.725)),
            ("z", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 2.725).abs() < 0.1, "T_CMB(z=0) ≈ 2.725 K, got {v}");
}

#[test]
fn cmb_temperature_scales_with_z() {
    let t0 = scalar(run_astro(
        "cmb_temperature",
        vec![
            ("t0", ParameterValue::Scalar(2.725)),
            ("z", ParameterValue::Scalar(0.0)),
        ],
    ));
    let t1 = scalar(run_astro(
        "cmb_temperature",
        vec![
            ("t0", ParameterValue::Scalar(2.725)),
            ("z", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(
        (t1 / t0 - 2.0).abs() < 0.01,
        "T(z=1) = 2·T(0), got {}",
        t1 / t0
    );
}

#[test]
fn e_z_lcdm_today() {
    let v = scalar(run_astro(
        "e_z_lcdm",
        vec![
            ("z", ParameterValue::Scalar(0.0)),
            ("omega_m", ParameterValue::Scalar(0.3)),
            ("omega_lambda", ParameterValue::Scalar(0.7)),
        ],
    ));
    assert!((v - 1.0).abs() < 1e-10, "E(z=0) = 1, got {v}");
}

#[test]
fn age_of_universe() {
    let v = scalar(run_astro(
        "age_lcdm",
        vec![
            ("h0", ParameterValue::Scalar(67.4)),
            ("omega_m", ParameterValue::Scalar(0.315)),
        ],
    ));
    assert!(v > 12.0 && v < 15.0, "age ≈ 13.8 Gyr, got {v} Gyr");
}

#[test]
fn hubble_at_z_matches_h0_times_ez_lcdm() {
    let h0 = 67.4;
    let omega_m = 0.315;
    let z = 1.2;
    let ez = scalar(run_astro(
        "e_z_lcdm",
        vec![
            ("omega_m", ParameterValue::Scalar(omega_m)),
            ("z", ParameterValue::Scalar(z)),
        ],
    ));
    let hz = scalar(run_astro(
        "hubble_at_z_lcdm",
        vec![
            ("h0", ParameterValue::Scalar(h0)),
            ("omega_m", ParameterValue::Scalar(omega_m)),
            ("z", ParameterValue::Scalar(z)),
        ],
    ));
    assert_close_rel(hz, h0 * ez, 1e-12, "hubble_at_z_lcdm identity");
}

#[test]
fn luminosity_angular_distance_duality_from_z() {
    let h0 = 67.4;
    let omega_m = 0.315;
    let z = 0.9;
    let d_l = scalar(run_astro(
        "luminosity_distance_from_z",
        vec![
            ("h0", ParameterValue::Scalar(h0)),
            ("omega_m", ParameterValue::Scalar(omega_m)),
            ("z", ParameterValue::Scalar(z)),
        ],
    ));
    let d_a = scalar(run_astro(
        "angular_diameter_distance_from_z",
        vec![
            ("h0", ParameterValue::Scalar(h0)),
            ("omega_m", ParameterValue::Scalar(omega_m)),
            ("z", ParameterValue::Scalar(z)),
        ],
    ));
    assert_close_rel(d_l / d_a, (1.0 + z) * (1.0 + z), 1e-8, "distance duality");
}

#[test]
fn comoving_distance_increases_with_redshift() {
    let h0 = 67.4;
    let omega_m = 0.315;
    let d1 = scalar(run_astro(
        "comoving_distance_from_z",
        vec![
            ("h0", ParameterValue::Scalar(h0)),
            ("omega_m", ParameterValue::Scalar(omega_m)),
            ("z", ParameterValue::Scalar(0.5)),
        ],
    ));
    let d2 = scalar(run_astro(
        "comoving_distance_from_z",
        vec![
            ("h0", ParameterValue::Scalar(h0)),
            ("omega_m", ParameterValue::Scalar(omega_m)),
            ("z", ParameterValue::Scalar(1.0)),
        ],
    ));
    let d3 = scalar(run_astro(
        "comoving_distance_from_z",
        vec![
            ("h0", ParameterValue::Scalar(h0)),
            ("omega_m", ParameterValue::Scalar(omega_m)),
            ("z", ParameterValue::Scalar(2.0)),
        ],
    ));
    assert!(d1 < d2 && d2 < d3);
}

#[test]
fn relativistic_redshift_velocity_roundtrip() {
    let v = 1.5e8;
    let z = scalar(run_astro(
        "relativistic_redshift",
        vec![("v", ParameterValue::Scalar(v))],
    ));
    let v_back = scalar(run_astro(
        "velocity_from_redshift",
        vec![("z", ParameterValue::Scalar(z))],
    ));
    assert!((v_back - v).abs() / v < 1e-10);
}

#[test]
fn age_decreases_with_redshift() {
    let h0 = 67.4;
    let omega_m = 0.315;
    let age0 = scalar(run_astro(
        "age_at_z",
        vec![
            ("h0", ParameterValue::Scalar(h0)),
            ("omega_m", ParameterValue::Scalar(omega_m)),
            ("z", ParameterValue::Scalar(0.0)),
        ],
    ));
    let age1 = scalar(run_astro(
        "age_at_z",
        vec![
            ("h0", ParameterValue::Scalar(h0)),
            ("omega_m", ParameterValue::Scalar(omega_m)),
            ("z", ParameterValue::Scalar(1.0)),
        ],
    ));
    let age3 = scalar(run_astro(
        "age_at_z",
        vec![
            ("h0", ParameterValue::Scalar(h0)),
            ("omega_m", ParameterValue::Scalar(omega_m)),
            ("z", ParameterValue::Scalar(3.0)),
        ],
    ));
    assert!(age0 > age1 && age1 > age3);
}

#[test]
fn lookback_time_increases_with_redshift() {
    let h0 = 67.4;
    let omega_m = 0.315;
    let t1 = scalar(run_astro(
        "lookback_time_from_z",
        vec![
            ("h0", ParameterValue::Scalar(h0)),
            ("omega_m", ParameterValue::Scalar(omega_m)),
            ("z", ParameterValue::Scalar(0.2)),
        ],
    ));
    let t2 = scalar(run_astro(
        "lookback_time_from_z",
        vec![
            ("h0", ParameterValue::Scalar(h0)),
            ("omega_m", ParameterValue::Scalar(omega_m)),
            ("z", ParameterValue::Scalar(1.0)),
        ],
    ));
    let t3 = scalar(run_astro(
        "lookback_time_from_z",
        vec![
            ("h0", ParameterValue::Scalar(h0)),
            ("omega_m", ParameterValue::Scalar(omega_m)),
            ("z", ParameterValue::Scalar(3.0)),
        ],
    ));
    assert!(t1 < t2 && t2 < t3);
}

#[test]
fn hubble_ez_identity_table_driven() {
    let h0 = 67.4;
    let omega_m = 0.315;
    for z in [0.0, 0.1, 0.5, 1.0, 2.0, 5.0] {
        let ez = scalar(run_astro(
            "e_z_lcdm",
            vec![
                ("omega_m", ParameterValue::Scalar(omega_m)),
                ("z", ParameterValue::Scalar(z)),
            ],
        ));
        let hz = scalar(run_astro(
            "hubble_at_z_lcdm",
            vec![
                ("h0", ParameterValue::Scalar(h0)),
                ("omega_m", ParameterValue::Scalar(omega_m)),
                ("z", ParameterValue::Scalar(z)),
            ],
        ));
        assert_close_rel(hz, h0 * ez, 1e-12, "table-driven hubble/ez identity");
    }
}

#[test]
fn distance_duality_table_driven() {
    let h0 = 67.4;
    let omega_m = 0.315;
    for z in [0.01, 0.1, 0.3, 0.7, 1.2, 2.0] {
        let d_l = scalar(run_astro(
            "luminosity_distance_from_z",
            vec![
                ("h0", ParameterValue::Scalar(h0)),
                ("omega_m", ParameterValue::Scalar(omega_m)),
                ("z", ParameterValue::Scalar(z)),
            ],
        ));
        let d_a = scalar(run_astro(
            "angular_diameter_distance_from_z",
            vec![
                ("h0", ParameterValue::Scalar(h0)),
                ("omega_m", ParameterValue::Scalar(omega_m)),
                ("z", ParameterValue::Scalar(z)),
            ],
        ));
        assert_close_rel(
            d_l / d_a,
            (1.0 + z) * (1.0 + z),
            1e-8,
            "table-driven distance duality",
        );
    }
}

#[test]
fn e_z_lcdm_sweep_monotonic_for_positive_z() {
    let omega_m = 0.315;
    let mut prev = scalar(run_astro(
        "e_z_lcdm",
        vec![
            ("omega_m", ParameterValue::Scalar(omega_m)),
            ("z", ParameterValue::Scalar(0.0)),
        ],
    ));
    for i in 1..=20 {
        let z = i as f64 * 0.25;
        let cur = scalar(run_astro(
            "e_z_lcdm",
            vec![
                ("omega_m", ParameterValue::Scalar(omega_m)),
                ("z", ParameterValue::Scalar(z)),
            ],
        ));
        assert!(
            cur > prev,
            "E(z) should increase with z: z={z}, prev={prev}, cur={cur}"
        );
        prev = cur;
    }
}

#[test]
fn comoving_distance_zero_at_zero_redshift() {
    let d0 = scalar(run_astro(
        "comoving_distance_from_z",
        vec![
            ("h0", ParameterValue::Scalar(67.4)),
            ("omega_m", ParameterValue::Scalar(0.315)),
            ("z", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!(d0.abs() < 1e-12);
}

#[test]
fn astronomy_unknown_function_returns_error() {
    let exp = Experiment::new(DomainType::Astronomy, "unknown_astronomy_function");
    let result = ExperimentRunner::new().run(&exp);
    assert!(result.is_err());
}

#[test]
fn astronomy_missing_required_parameter_returns_error() {
    let exp = Experiment::new(DomainType::Astronomy, "hubble_velocity")
        .param("h0", ParameterValue::Scalar(70.0));
    let result = ExperimentRunner::new().run(&exp);
    assert!(result.is_err());
}
