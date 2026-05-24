use sciforge_hub::prelude::*;

#[test]
fn ev_to_joule() {
    assert!((constants::EV_TO_JOULE - 1.602_176_634e-19).abs() < 1e-27);
}

#[test]
fn ev_roundtrip() {
    let j = 5.0 * constants::EV_TO_JOULE;
    let ev = j * constants::JOULE_TO_EV;
    assert!((ev - 5.0).abs() < 1e-10);
}

#[test]
fn calorie_to_joule() {
    assert!((constants::CALORIE_TO_JOULE - 4.184).abs() < 1e-6);
}

#[test]
fn atm_to_pascal() {
    assert!((constants::ATM_TO_PASCAL - 101_325.0).abs() < 1.0);
}

#[test]
fn degree_rad_roundtrip() {
    let rad = 180.0 * constants::DEGREE_TO_RAD;
    assert!((rad - std::f64::consts::PI).abs() < 1e-12);
    let deg = rad * constants::RAD_TO_DEGREE;
    assert!((deg - 180.0).abs() < 1e-10);
}

#[test]
fn ev_joule_roundtrip_multiple_values() {
    let values = [1e-9, 1.0, 2.5, 1e3, 1e9];
    for ev0 in values {
        let j = ev0 * constants::EV_TO_JOULE;
        let ev1 = j * constants::JOULE_TO_EV;
        assert!((ev1 - ev0).abs() / ev0.max(1.0) < 1e-12);
    }
}

#[test]
fn angle_conversion_inverse_identity() {
    let angles = [-720.0, -90.0, 0.0, 30.0, 180.0, 450.0];
    for deg0 in angles {
        let rad = deg0 * constants::DEGREE_TO_RAD;
        let deg1 = rad * constants::RAD_TO_DEGREE;
        assert!((deg1 - deg0).abs() < 1e-10);
    }
}
