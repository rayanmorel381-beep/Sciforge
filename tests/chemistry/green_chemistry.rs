use sciforge_hub::prelude::*;

fn run_chem(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Chemistry, name);
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
fn gc_atom_economy() {
    let v = scalar(run_chem(
        "gc_atom_economy",
        vec![
            ("mw_product", ParameterValue::Scalar(180.0)),
            ("mw_all_products", ParameterValue::Scalar(360.0)),
        ],
    ));
    assert!((v - 50.0).abs() < 1e-6, "AE=180/360*100=50%, got {v}");
}

#[test]
fn e_factor() {
    let v = scalar(run_chem(
        "e_factor",
        vec![
            ("mass_waste", ParameterValue::Scalar(90.0)),
            ("mass_product", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!((v - 9.0).abs() < 1e-8, "E=90/10=9, got {v}");
}

#[test]
fn reaction_mass_efficiency() {
    let v = scalar(run_chem(
        "reaction_mass_efficiency",
        vec![
            ("mass_product", ParameterValue::Scalar(50.0)),
            ("total_mass_reactants", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!((v - 50.0).abs() < 1e-6, "RME=50%, got {v}");
}
