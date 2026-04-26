use sciforge::hub::prelude::*;

#[test]
fn proton_mass() {
    assert!((constants::PROTON_MASS_KG - 1.672_621_923_69e-27).abs() < 1e-36);
}

#[test]
fn neutron_mass() {
    assert!((constants::NEUTRON_MASS_KG - 1.674_927_498_04e-27).abs() < 1e-36);
}

#[test]
fn electron_mass() {
    assert!((constants::ELECTRON_MASS_KG - 9.109_383_701_5e-31).abs() < 1e-40);
}

#[test]
fn proton_heavier_than_electron() {
    assert!(
        std::hint::black_box(constants::PROTON_MASS_KG)
            > std::hint::black_box(constants::ELECTRON_MASS_KG) * 1800.0
    );
}

#[test]
fn amu_to_kg() {
    assert!((constants::AMU_TO_KG - 1.660_539_066_60e-27).abs() < 1e-36);
}

#[test]
fn bohr_radius() {
    assert!((constants::BOHR_RADIUS - 5.291_772_109_03e-11).abs() < 1e-20);
}

#[test]
fn compton_wavelength() {
    assert!((constants::COMPTON_WAVELENGTH - 2.426_310_238_67e-12).abs() < 1e-21);
}
