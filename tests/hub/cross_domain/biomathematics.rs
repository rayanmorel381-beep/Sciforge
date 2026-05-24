use sciforge_hub::prelude::*;

fn run(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Biomathematics, name);
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
fn logistic_growth_rate() {
    let v = scalar(run(
        "logistic_growth_rate",
        vec![
            ("r", ParameterValue::Scalar(0.5)),
            ("carrying_capacity", ParameterValue::Scalar(1000.0)),
            ("population", ParameterValue::Scalar(500.0)),
        ],
    ));
    assert!(
        (v - 125.0).abs() < 1e-6,
        "dN/dt=r*N*(1-N/K)=0.5*500*0.5=125, got {v}"
    );
}

#[test]
fn basic_reproduction_number() {
    let v = scalar(run(
        "basic_reproduction_number",
        vec![
            ("beta", ParameterValue::Scalar(0.3)),
            ("gamma", ParameterValue::Scalar(0.1)),
        ],
    ));
    assert!((v - 3.0).abs() < 1e-6, "R0=β/γ=3, got {v}");
}

#[test]
fn shannon_diversity_index() {
    let v = scalar(run(
        "shannon_diversity_index",
        vec![("proportions", ParameterValue::Vector(vec![0.5, 0.5]))],
    ));
    let expected = 2.0_f64.ln();
    assert!(
        (v - expected).abs() < 1e-6,
        "H=ln(2) for equal proportions, got {v}"
    );
}

#[test]
fn molecular_clock_distance() {
    let v = scalar(run(
        "molecular_clock_distance",
        vec![
            ("substitution_rate", ParameterValue::Scalar(1e-8)),
            ("time", ParameterValue::Scalar(1e6)),
        ],
    ));
    assert!((v - 0.02).abs() < 1e-6, "d=2*rate*time=0.02, got {v}");
}
