use sciforge::hub::prelude::*;

#[test]
fn astronomical_unit() {
    assert!((constants::AU - 1.495_978_707e11).abs() < 1e5);
}

#[test]
fn parsec() {
    assert!((constants::PARSEC - 3.085_677_581e16).abs() < 1e10);
}

#[test]
fn light_year() {
    assert!((constants::LIGHT_YEAR - 9.460_730_472_580_8e15).abs() < 1e9);
}

#[test]
fn solar_mass() {
    assert!((constants::SOLAR_MASS - 1.989_1e30).abs() < 1e27);
}

#[test]
fn earth_mass() {
    assert!((constants::EARTH_MASS - 5.972_37e24).abs() < 1e21);
}

#[test]
fn hubble_constant() {
    assert!((constants::HUBBLE_CONSTANT - 67.4).abs() < 1.0);
}

#[test]
fn cmb_temperature() {
    assert!((constants::CMB_TEMPERATURE - 2.725).abs() < 0.01);
}

#[test]
fn parsec_greater_than_ly() {
    assert!(std::hint::black_box(constants::PARSEC) > std::hint::black_box(constants::LIGHT_YEAR));
}
