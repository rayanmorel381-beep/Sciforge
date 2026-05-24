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
fn bond_order() {
    let v = scalar(run_chem(
        "bond_order",
        vec![
            ("bonding_electrons", ParameterValue::Scalar(8.0)),
            ("antibonding_electrons", ParameterValue::Scalar(2.0)),
        ],
    ));
    assert!((v - 3.0).abs() < 1e-8, "BO=(8-2)/2=3, got {v}");
}

#[test]
fn formal_charge() {
    let o = run_chem(
        "formal_charge",
        vec![
            ("valence", ParameterValue::Integer(6)),
            ("lone_pair", ParameterValue::Integer(4)),
            ("bonding", ParameterValue::Integer(4)),
        ],
    );
    match o {
        RunOutput::Integer(v) => assert!(v == 0, "FC=6-4-4/2=0, got {v}"),
        other => panic!("expected Integer, got {other:?}"),
    }
}

#[test]
fn molar_mass() {
    let v = scalar(run_chem(
        "molar_mass",
        vec![
            ("atomic_masses", ParameterValue::Vector(vec![1.008, 15.999])),
            ("counts", ParameterValue::Vector(vec![2.0, 1.0])),
        ],
    ));
    assert!((v - 18.015).abs() < 0.1, "M(H2O)≈18.015, got {v}");
}

#[test]
fn dipole_moment() {
    let o = run_chem(
        "dipole_moment",
        vec![(
            "charges",
            ParameterValue::Matrix(vec![
                vec![1.6e-19, 1e-10, 0.0, 0.0],
                vec![-1.6e-19, 0.0, 0.0, 0.0],
            ]),
        )],
    );
    match o {
        RunOutput::Triple(x, y, z) => {
            let mag = (x * x + y * y + z * z).sqrt();
            assert!(mag > 0.0, "dipole moment should be positive, got {mag}");
        }
        other => panic!("expected Triple, got {other:?}"),
    }
}
