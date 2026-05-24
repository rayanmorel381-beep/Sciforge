use sciforge_hub::prelude::*;
use sciforge_hub::tools::validation::{check_monotonicity, check_nan_safety};

fn run(domain: DomainType, function: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(domain, function);
    for (key, value) in params {
        exp = exp.param(key, value);
    }
    ExperimentRunner::new()
        .run(&exp)
        .unwrap_or_else(|_| panic!("dispatch '{function}' failed"))
}

fn scalar(output: RunOutput) -> f64 {
    match output {
        RunOutput::Scalar(value) => value,
        other => panic!("expected scalar output, got {other:?}"),
    }
}

#[test]
fn monotonicity_properties_hold_for_core_models() {
    let c = constants::C;
    assert!(check_monotonicity(
        DomainType::Physics,
        "gamma_factor",
        vec![("v", 0.0)],
        "v",
        &[0.0, 0.1 * c, 0.25 * c, 0.5 * c, 0.75 * c, 0.9 * c],
        true,
    ));
    assert!(check_monotonicity(
        DomainType::Chemistry,
        "ph_strong_acid",
        vec![("concentration", 1e-6)],
        "concentration",
        &[1e-6, 1e-5, 1e-4, 1e-3, 1e-2],
        false,
    ));
    assert!(check_monotonicity(
        DomainType::Biology,
        "michaelis_menten",
        vec![("s", 0.0), ("vmax", 100.0), ("km", 5.0)],
        "s",
        &[0.0, 0.5, 1.0, 5.0, 10.0, 100.0],
        true,
    ));
    assert!(check_monotonicity(
        DomainType::Astronomy,
        "escape_velocity",
        vec![("mu", 3.986e14), ("r", 6.371e6)],
        "r",
        &[6.371e6, 8.0e6, 1.0e7, 2.0e7, 4.2e7],
        false,
    ));
    assert!(check_monotonicity(
        DomainType::Geology,
        "radioactive_decay",
        vec![("n0", 1.0), ("lambda", 1e-4), ("t", 0.0)],
        "t",
        &[0.0, 1.0, 10.0, 100.0, 1_000.0],
        false,
    ));
    assert!(check_monotonicity(
        DomainType::Meteorology,
        "barometric_formula",
        vec![
            ("p0", 101_325.0),
            ("m", 0.028_964_4),
            ("g", 9.806_65),
            ("h", 0.0),
            ("t", 288.15),
        ],
        "h",
        &[0.0, 100.0, 500.0, 1_000.0, 5_000.0],
        false,
    ));
    assert!(check_monotonicity(
        DomainType::Maths,
        "prob_uniform_cdf",
        vec![("x", 0.0), ("a", 0.0), ("b", 1.0)],
        "x",
        &[0.0, 0.1, 0.25, 0.5, 0.75, 1.0],
        true,
    ));
}

#[test]
fn randomized_property_smoke_tests_stay_finite() {
    let mut rng = Rng::new(0x5c1f0fce_u64);
    for _ in 0..64 {
        let proper_time = rng.uniform(0.0, 10.0);
        let v = rng.uniform(0.0, 0.95 * constants::C);
        let gamma = scalar(run(
            DomainType::Physics,
            "time_dilation",
            vec![
                ("proper_time", ParameterValue::Scalar(proper_time)),
                ("v", ParameterValue::Scalar(v)),
            ],
        ));
        assert!(gamma.is_finite());
        assert!(gamma >= proper_time);

        let concentration = rng.uniform(1e-6, 1.0);
        let ph = scalar(run(
            DomainType::Chemistry,
            "ph_strong_acid",
            vec![("concentration", ParameterValue::Scalar(concentration))],
        ));
        assert!(ph.is_finite());
        assert!(ph >= -1e-12);

        let substrate = rng.uniform(0.0, 1000.0);
        let vmax = rng.uniform(1.0, 500.0);
        let km = rng.uniform(0.1, 100.0);
        let rate = scalar(run(
            DomainType::Biology,
            "michaelis_menten",
            vec![
                ("s", ParameterValue::Scalar(substrate)),
                ("vmax", ParameterValue::Scalar(vmax)),
                ("km", ParameterValue::Scalar(km)),
            ],
        ));
        assert!(rate.is_finite());
        assert!(rate >= 0.0);
        assert!(rate <= vmax + 1e-9);

        let radius = rng.uniform(6.4e6, 4.0e8);
        let escape = scalar(run(
            DomainType::Astronomy,
            "escape_velocity",
            vec![
                ("mu", ParameterValue::Scalar(3.986e14)),
                ("r", ParameterValue::Scalar(radius)),
            ],
        ));
        assert!(escape.is_finite());
        assert!(escape > 0.0);

        let decay = scalar(run(
            DomainType::Geology,
            "radioactive_decay",
            vec![
                ("n0", ParameterValue::Scalar(1.0)),
                ("lambda", ParameterValue::Scalar(rng.uniform(1e-8, 1e-2))),
                ("t", ParameterValue::Scalar(rng.uniform(0.0, 1e6))),
            ],
        ));
        assert!(decay.is_finite());
        assert!(decay >= 0.0);
        assert!(decay <= 1.0 + 1e-12);

        let pressure = scalar(run(
            DomainType::Meteorology,
            "barometric_formula",
            vec![
                ("p0", ParameterValue::Scalar(101_325.0)),
                ("m", ParameterValue::Scalar(0.028_964_4)),
                ("g", ParameterValue::Scalar(9.806_65)),
                ("h", ParameterValue::Scalar(rng.uniform(0.0, 10_000.0))),
                ("t", ParameterValue::Scalar(rng.uniform(230.0, 320.0))),
            ],
        ));
        assert!(pressure.is_finite());
        assert!(pressure > 0.0);

        let cdf = scalar(run(
            DomainType::Maths,
            "prob_normal_cdf",
            vec![
                ("x", ParameterValue::Scalar(rng.uniform(-6.0, 6.0))),
                ("mu", ParameterValue::Scalar(0.0)),
                ("sigma", ParameterValue::Scalar(rng.uniform(0.1, 5.0))),
            ],
        ));
        assert!(cdf.is_finite());
        assert!((0.0..=1.0).contains(&cdf));
    }
}

#[test]
fn nan_safety_helpers_cover_all_scientific_domains() {
    assert!(check_nan_safety(
        DomainType::Physics,
        "carnot_efficiency",
        vec![("t_hot", 600.0), ("t_cold", 300.0)],
    ));
    assert!(check_nan_safety(
        DomainType::Chemistry,
        "ideal_gas_pressure",
        vec![("n", 1.0), ("t", 300.0), ("v", 0.0224)],
    ));
    assert!(check_nan_safety(
        DomainType::Biology,
        "michaelis_menten",
        vec![("s", 5.0), ("vmax", 100.0), ("km", 2.0)],
    ));
    assert!(check_nan_safety(
        DomainType::Astronomy,
        "kepler_period",
        vec![("a", 7.0e6), ("mu", 3.986e14)],
    ));
    assert!(check_nan_safety(
        DomainType::Geology,
        "plate_velocity",
        vec![("distance", 100.0), ("time", 2.0)],
    ));
    assert!(check_nan_safety(
        DomainType::Meteorology,
        "dew_point",
        vec![("t", 20.0), ("rh", 0.5)],
    ));
    assert!(check_nan_safety(
        DomainType::Maths,
        "prob_uniform_pdf",
        vec![("x", 0.5), ("a", 0.0), ("b", 1.0)],
    ));
}
