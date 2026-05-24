use sciforge_hub::prelude::*;

fn run_phys(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Physics, name);
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
fn electric_field_point_charge() {
    let out = run_phys(
        "electric_field_point_charge",
        vec![
            ("q", ParameterValue::Scalar(1e-6)),
            ("r", ParameterValue::Vector(vec![1.0, 0.0, 0.0])),
        ],
    );
    match out {
        RunOutput::Triple(ex, ey, ez) => {
            assert!(ex > 0.0, "E-field x-component should be positive, got {ex}");
            assert!(ey.abs() < 1e-6);
            assert!(ez.abs() < 1e-6);
        }
        other => panic!("expected Triple, got {other:?}"),
    }
}

#[test]
fn capacitance_parallel_plate() {
    let v = scalar(run_phys(
        "capacitance_parallel_plate",
        vec![
            ("epsilon_r", ParameterValue::Scalar(1.0)),
            ("area", ParameterValue::Scalar(1.0)),
            ("distance", ParameterValue::Scalar(1e-3)),
        ],
    ));
    assert!(v > 0.0, "capacitance should be positive, got {v}");
}

#[test]
fn skin_depth() {
    let v = scalar(run_phys(
        "skin_depth",
        vec![
            ("conductivity", ParameterValue::Scalar(1.68e-8)),
            ("frequency", ParameterValue::Scalar(1e6)),
            ("mu_r", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(v > 0.0, "skin depth should be positive, got {v}");
}
