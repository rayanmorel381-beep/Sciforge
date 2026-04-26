use sciforge::hub::prelude::*;

fn run_bio(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Biology, name);
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
fn vaccine_efficacy() {
    let v = scalar(run_bio(
        "vaccine_efficacy",
        vec![
            ("arr_vacc", ParameterValue::Scalar(0.1)),
            ("arr_placebo", ParameterValue::Scalar(0.5)),
        ],
    ));
    assert!((v - 0.8).abs() < 1e-8, "VE=(0.5-0.1)/0.5=0.8, got {v}");
}

#[test]
fn herd_immunity_fraction() {
    let v = scalar(run_bio(
        "herd_immunity_fraction",
        vec![("r0", ParameterValue::Scalar(4.0))],
    ));
    assert!((v - 0.75).abs() < 1e-8, "1-1/R0=0.75, got {v}");
}

#[test]
fn clonal_expansion() {
    let v = scalar(run_bio(
        "clonal_expansion",
        vec![
            ("n0", ParameterValue::Scalar(1.0)),
            ("proliferation_rate", ParameterValue::Scalar(0.5)),
            ("death_rate", ParameterValue::Scalar(0.0)),
            ("t", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!(v > 1.0, "cells should expand, got {v}");
}

#[test]
fn antibody_titer() {
    let v = scalar(run_bio(
        "antibody_titer",
        vec![
            ("dilution_factor", ParameterValue::Scalar(2.0)),
            ("endpoint_dilution", ParameterValue::Integer(8)),
        ],
    ));
    assert!(
        v > 0.0 && v.is_finite(),
        "titer should be positive, got {v}"
    );
}
