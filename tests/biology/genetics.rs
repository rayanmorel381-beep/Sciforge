use sciforge_hub::prelude::*;

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
fn hardy_weinberg() {
    let o = run_bio("hardy_weinberg", vec![("p", ParameterValue::Scalar(0.6))]);
    match o {
        RunOutput::Triple(p2, pq2, q2) => {
            assert!((p2 - 0.36).abs() < 1e-8, "p²=0.36, got {p2}");
            assert!((pq2 - 0.48).abs() < 1e-8, "2pq=0.48, got {pq2}");
            assert!((q2 - 0.16).abs() < 1e-8, "q²=0.16, got {q2}");
        }
        other => panic!("expected Triple, got {other:?}"),
    }
}

#[test]
fn broad_sense_heritability() {
    let v = scalar(run_bio(
        "broad_sense_heritability",
        vec![
            ("genetic_variance", ParameterValue::Scalar(40.0)),
            ("phenotypic_variance", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!((v - 0.4).abs() < 1e-8, "H²=Vg/Vp=0.4, got {v}");
}

#[test]
fn effective_population_size() {
    let v = scalar(run_bio(
        "effective_population_size",
        vec![
            ("n_males", ParameterValue::Scalar(100.0)),
            ("n_females", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!((v - 200.0).abs() < 1.0, "Ne=4NmNf/(Nm+Nf)=200, got {v}");
}

#[test]
fn mutation_selection_balance() {
    let v = scalar(run_bio(
        "mutation_selection_balance",
        vec![
            ("mu", ParameterValue::Scalar(1e-5)),
            ("s", ParameterValue::Scalar(0.01)),
        ],
    ));
    let expected = 1e-5 / 0.01;
    assert!((v - expected).abs() < 1e-8, "q=mu/s=0.001, got {v}");
}
