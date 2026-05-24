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
fn crater_diameter() {
    let v = scalar(run_astro(
        "crater_diameter",
        vec![
            ("rho_i", ParameterValue::Scalar(3000.0)),
            ("d_p", ParameterValue::Scalar(100.0)),
            ("v", ParameterValue::Scalar(20000.0)),
            ("g", ParameterValue::Scalar(9.81)),
            ("rho_t", ParameterValue::Scalar(2500.0)),
        ],
    ));
    assert!(v > 0.0, "crater diameter should be positive, got {v}");
}

#[test]
fn fireball_radius() {
    let v = scalar(run_astro(
        "fireball_radius",
        vec![("e_kt", ParameterValue::Scalar(1.0))],
    ));
    assert!(v > 0.0, "fireball radius should be positive, got {v}");
}

#[test]
fn impact_velocity() {
    let v = scalar(run_astro(
        "impact_velocity",
        vec![
            ("v_inf", ParameterValue::Scalar(20000.0)),
            ("v_esc", ParameterValue::Scalar(11200.0)),
        ],
    ));
    assert!(v > 20000.0, "impact velocity > v_inf, got {v}");
}
