mod astrobiology;
mod astrochemistry;
mod astrophysics;
mod atmospheric_chemistry;
mod atmospheric_physics;
mod biochemistry;
mod biomathematics;
mod biophysics;
mod geochemistry;
mod geophysics;
mod mathematical_physics;
mod planetary_geology;

use sciforge::hub::prelude::*;

#[test]
fn campaign_physics_and_chemistry() {
    let runner = ExperimentRunner::new();

    let gas_pressure = Experiment::new(DomainType::Chemistry, "ideal_gas_pressure")
        .param("n", ParameterValue::Scalar(1.0))
        .param("t", ParameterValue::Scalar(300.0))
        .param("v", ParameterValue::Scalar(0.0224));

    let carnot = Experiment::new(DomainType::Physics, "carnot_efficiency")
        .param("t_hot", ParameterValue::Scalar(500.0))
        .param("t_cold", ParameterValue::Scalar(300.0));

    let campaign = Campaign::new("thermo_cross")
        .add_step("gas_pressure", gas_pressure)
        .add_step("carnot", carnot);

    let result = campaign.run(&runner).unwrap();
    assert_eq!(result.len(), 2);
    let p = result.get("gas_pressure").unwrap();
    if let RunOutput::Scalar(v) = p {
        assert!(*v > 0.0, "pressure should be positive");
    } else {
        panic!("expected scalar for gas_pressure");
    }
    let c = result.get("carnot").unwrap();
    if let RunOutput::Scalar(v) = c {
        assert!(*v > 0.0 && *v < 1.0, "Carnot efficiency in (0,1)");
    } else {
        panic!("expected scalar for carnot");
    }
}

#[test]
fn campaign_astronomy_and_physics() {
    let runner = ExperimentRunner::new();

    let escape = Experiment::new(DomainType::Astronomy, "escape_velocity")
        .param("mu", ParameterValue::Scalar(3.986e14))
        .param("r", ParameterValue::Scalar(6.371e6));

    let gamma = Experiment::new(DomainType::Physics, "gamma_factor")
        .param("v", ParameterValue::Scalar(1000.0));

    let campaign = Campaign::new("astro_physics")
        .add_step("escape", escape)
        .add_step("gamma", gamma);

    let result = campaign.run(&runner).unwrap();
    assert_eq!(result.len(), 2);
    let scalars = result.scalars();
    assert_eq!(scalars.len(), 2);
    for (name, val) in &scalars {
        assert!(val.is_finite(), "{name} should be finite");
    }
}

#[test]
fn campaign_result_scalars() {
    let runner = ExperimentRunner::new();
    let campaign = Campaign::new("scalars_test")
        .add_step(
            "c1",
            Experiment::new(DomainType::Physics, "carnot_efficiency")
                .param("t_hot", ParameterValue::Scalar(600.0))
                .param("t_cold", ParameterValue::Scalar(300.0)),
        )
        .add_step(
            "c2",
            Experiment::new(DomainType::Physics, "carnot_efficiency")
                .param("t_hot", ParameterValue::Scalar(800.0))
                .param("t_cold", ParameterValue::Scalar(300.0)),
        );

    let result = campaign.run(&runner).unwrap();
    let scalars = result.scalars();
    assert_eq!(scalars.len(), 2);
    let e1 = scalars.iter().find(|(n, _)| *n == "c1").unwrap().1;
    let e2 = scalars.iter().find(|(n, _)| *n == "c2").unwrap().1;
    assert!(e2 > e1, "higher T_hot should give higher efficiency");
}

#[test]
fn campaign_empty() {
    let runner = ExperimentRunner::new();
    let campaign = Campaign::new("empty");
    let result = campaign.run(&runner).unwrap();
    assert!(result.is_empty());
    assert_eq!(result.len(), 0);
}

#[test]
fn validation_single_case() {
    let case = ValidationCase::new(
        "carnot_efficiency_50pct",
        DomainType::Physics,
        "carnot_efficiency",
        vec![("t_hot", 600.0), ("t_cold", 300.0)],
        0.5,
        1e-6,
        "thermodynamics textbook",
    );
    let report = run_validation(std::slice::from_ref(&case));
    assert!(
        report.all_passed(),
        "carnot 600/300 should give 0.5: {:?}",
        report.failures()
    );
    assert_eq!(report.total(), 1);
    assert_eq!(report.passed_count(), 1);
}

#[test]
fn validation_markdown_output() {
    let case = ValidationCase::new(
        "test_case",
        DomainType::Physics,
        "carnot_efficiency",
        vec![("t_hot", 600.0), ("t_cold", 300.0)],
        0.5,
        1e-6,
        "test",
    );
    let report = run_validation(std::slice::from_ref(&case));
    let md = report.to_markdown();
    assert!(
        md.contains("Validation Report"),
        "markdown should have header"
    );
}

#[test]
fn cross_domain_earth_escape_velocity() {
    let runner = ExperimentRunner::new();
    let exp = Experiment::new(DomainType::Astronomy, "escape_velocity")
        .param("mu", ParameterValue::Scalar(3.986e14))
        .param("r", ParameterValue::Scalar(6.371e6));
    let result = runner.run(&exp).unwrap();
    if let RunOutput::Scalar(v) = result {
        assert!(
            (v - 11_186.0).abs() < 100.0,
            "Earth escape ≈ 11186 m/s, got {v}"
        );
    } else {
        panic!("expected scalar");
    }
}

#[test]
fn cross_domain_biology_and_chemistry() {
    let runner = ExperimentRunner::new();

    let enzyme = Experiment::new(DomainType::Biology, "michaelis_menten")
        .param("s", ParameterValue::Scalar(10.0))
        .param("vmax", ParameterValue::Scalar(100.0))
        .param("km", ParameterValue::Scalar(5.0));

    let ph = Experiment::new(DomainType::Chemistry, "ph_strong_acid")
        .param("concentration", ParameterValue::Scalar(0.01));

    let campaign = Campaign::new("bio_chem")
        .add_step("enzyme_rate", enzyme)
        .add_step("acid_ph", ph);

    let result = campaign.run(&runner).unwrap();
    assert_eq!(result.len(), 2);

    if let RunOutput::Scalar(rate) = result.get("enzyme_rate").unwrap() {
        let expected = 100.0 * 10.0 / (5.0 + 10.0);
        assert!(
            (*rate - expected).abs() < 0.1,
            "MM rate ≈ {expected}, got {rate}"
        );
    }
    if let RunOutput::Scalar(ph_val) = result.get("acid_ph").unwrap() {
        assert!(
            *ph_val > 1.0 && *ph_val < 3.0,
            "pH of 0.01 M strong acid ≈ 2.0, got {ph_val}"
        );
    }
}

#[test]
fn invalid_function_returns_error() {
    let runner = ExperimentRunner::new();
    let exp = Experiment::new(DomainType::Physics, "does_not_exist");
    let result = runner.run(&exp);
    assert!(result.is_err());
}

#[test]
fn validation_detects_bad_expected_value() {
    let bad_case = ValidationCase::new(
        "bad_carnot",
        DomainType::Physics,
        "carnot_efficiency",
        vec![("t_hot", 600.0), ("t_cold", 300.0)],
        0.9,
        1e-6,
        "intentional mismatch",
    );
    let report = run_validation(std::slice::from_ref(&bad_case));
    assert_eq!(report.total(), 1);
    assert_eq!(report.passed_count(), 0);
    assert!(!report.all_passed());
}
