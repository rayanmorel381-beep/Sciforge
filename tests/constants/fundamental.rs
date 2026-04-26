use sciforge::hub::prelude::*;

#[test]
fn speed_of_light() {
    assert!((constants::C - 299_792_458.0).abs() < 1.0);
}

#[test]
fn gravitational_constant() {
    assert!((constants::G - 6.674_30e-11).abs() < 1e-15);
}

#[test]
fn planck_constant() {
    assert!((constants::H - 6.626_070_15e-34).abs() < 1e-42);
}

#[test]
fn reduced_planck() {
    let expected = constants::H / (2.0 * std::f64::consts::PI);
    assert!((constants::HBAR - expected).abs() / expected < 1e-6);
}

#[test]
fn boltzmann_constant() {
    assert!((constants::K_B - 1.380_649e-23).abs() < 1e-29);
}

#[test]
fn avogadro_number() {
    assert!((constants::N_A - 6.022_140_76e23).abs() < 1e17);
}

#[test]
fn elementary_charge() {
    assert!((constants::E_CHARGE - 1.602_176_634e-19).abs() < 1e-27);
}

#[test]
fn stefan_boltzmann() {
    assert!((constants::SIGMA_SB - 5.670_374_419e-8).abs() < 1e-16);
}

#[test]
fn gas_constant() {
    assert!((constants::R_GAS - 8.314_462_618).abs() < 1e-6);
}

#[test]
fn coulomb_constant() {
    assert!(std::hint::black_box(constants::K_COULOMB) > 8.98e9);
}
