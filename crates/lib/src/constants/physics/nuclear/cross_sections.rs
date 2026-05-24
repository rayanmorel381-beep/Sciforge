#[derive(Debug, Clone, Copy)]
pub struct NeutronCrossSection {
    pub isotope: &'static str,
    pub mass_number: u32,
    pub sigma_capture_barn: f64,
    pub sigma_fission_barn: f64,
    pub sigma_scatter_barn: f64,
    pub sigma_total_barn: f64,
    pub neutron_energy_ev: f64,
}

pub const TABLE: &[NeutronCrossSection] = &[
    NeutronCrossSection { isotope: "H",   mass_number:   1, sigma_capture_barn:  0.3326, sigma_fission_barn: 0.0,    sigma_scatter_barn: 20.49,  sigma_total_barn:  20.82,   neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "D",   mass_number:   2, sigma_capture_barn:  0.000519,sigma_fission_barn: 0.0,    sigma_scatter_barn:  3.39,  sigma_total_barn:   3.39,   neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "He",  mass_number:   3, sigma_capture_barn:  5333.0, sigma_fission_barn: 0.0,    sigma_scatter_barn:  3.10,  sigma_total_barn: 5336.0,   neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "He",  mass_number:   4, sigma_capture_barn:  0.0,    sigma_fission_barn: 0.0,    sigma_scatter_barn:  0.76,  sigma_total_barn:    0.76,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Li",  mass_number:   6, sigma_capture_barn:  938.5,  sigma_fission_barn: 0.0,    sigma_scatter_barn:  0.97,  sigma_total_barn:  939.5,   neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Be",  mass_number:   9, sigma_capture_barn:  0.0076, sigma_fission_barn: 0.0,    sigma_scatter_barn:  6.15,  sigma_total_barn:    6.15,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "B",   mass_number:  10, sigma_capture_barn:  3837.0, sigma_fission_barn: 0.0,    sigma_scatter_barn:  2.20,  sigma_total_barn: 3839.0,   neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "C",   mass_number:  12, sigma_capture_barn:  0.0035, sigma_fission_barn: 0.0,    sigma_scatter_barn:  4.74,  sigma_total_barn:    4.74,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "N",   mass_number:  14, sigma_capture_barn:  1.91,   sigma_fission_barn: 0.0,    sigma_scatter_barn: 10.05,  sigma_total_barn:   11.96,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "O",   mass_number:  16, sigma_capture_barn:  0.00019,sigma_fission_barn: 0.0,    sigma_scatter_barn:  3.76,  sigma_total_barn:    3.76,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Na",  mass_number:  23, sigma_capture_barn:  0.530,  sigma_fission_barn: 0.0,    sigma_scatter_barn:  3.28,  sigma_total_barn:    3.81,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Mg",  mass_number:  24, sigma_capture_barn:  0.053,  sigma_fission_barn: 0.0,    sigma_scatter_barn:  3.13,  sigma_total_barn:    3.18,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Al",  mass_number:  27, sigma_capture_barn:  0.231,  sigma_fission_barn: 0.0,    sigma_scatter_barn:  1.41,  sigma_total_barn:    1.64,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Si",  mass_number:  28, sigma_capture_barn:  0.171,  sigma_fission_barn: 0.0,    sigma_scatter_barn:  2.04,  sigma_total_barn:    2.21,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Cl",  mass_number:  35, sigma_capture_barn: 43.6,    sigma_fission_barn: 0.0,    sigma_scatter_barn: 20.6,   sigma_total_barn:   64.20,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Fe",  mass_number:  56, sigma_capture_barn:  2.59,   sigma_fission_barn: 0.0,    sigma_scatter_barn: 11.62,  sigma_total_barn:   14.21,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Co",  mass_number:  59, sigma_capture_barn: 37.18,   sigma_fission_barn: 0.0,    sigma_scatter_barn:  6.0,   sigma_total_barn:   43.18,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Ni",  mass_number:  58, sigma_capture_barn:  4.50,   sigma_fission_barn: 0.0,    sigma_scatter_barn: 26.10,  sigma_total_barn:   30.60,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Cu",  mass_number:  63, sigma_capture_barn:  4.50,   sigma_fission_barn: 0.0,    sigma_scatter_barn:  5.20,  sigma_total_barn:    9.70,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Zr",  mass_number:  90, sigma_capture_barn:  0.014,  sigma_fission_barn: 0.0,    sigma_scatter_barn:  5.0,   sigma_total_barn:    5.01,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Ag",  mass_number: 109, sigma_capture_barn: 91.0,    sigma_fission_barn: 0.0,    sigma_scatter_barn:  4.7,   sigma_total_barn:   95.70,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Cd",  mass_number: 113, sigma_capture_barn: 20600.0, sigma_fission_barn: 0.0,    sigma_scatter_barn:  6.0,   sigma_total_barn: 20606.0,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "In",  mass_number: 115, sigma_capture_barn: 202.0,   sigma_fission_barn: 0.0,    sigma_scatter_barn:  2.0,   sigma_total_barn:  204.0,   neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Hf",  mass_number: 177, sigma_capture_barn: 373.0,   sigma_fission_barn: 0.0,    sigma_scatter_barn:  8.0,   sigma_total_barn:  381.0,   neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "W",   mass_number: 184, sigma_capture_barn:  1.70,   sigma_fission_barn: 0.0,    sigma_scatter_barn:  7.0,   sigma_total_barn:    8.70,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Au",  mass_number: 197, sigma_capture_barn: 98.65,   sigma_fission_barn: 0.0,    sigma_scatter_barn:  7.84,  sigma_total_barn:  106.49,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Hg",  mass_number: 199, sigma_capture_barn: 2150.0,  sigma_fission_barn: 0.0,    sigma_scatter_barn: 18.0,   sigma_total_barn: 2168.0,   neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Pb",  mass_number: 208, sigma_capture_barn:  0.00048,sigma_fission_barn: 0.0,    sigma_scatter_barn: 11.30,  sigma_total_barn:   11.30,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Th",  mass_number: 232, sigma_capture_barn:  7.40,   sigma_fission_barn: 0.0,    sigma_scatter_barn: 13.6,   sigma_total_barn:   20.94,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "U",   mass_number: 233, sigma_capture_barn: 45.5,    sigma_fission_barn: 531.1,  sigma_scatter_barn: 12.2,   sigma_total_barn:  588.8,   neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "U",   mass_number: 235, sigma_capture_barn: 98.81,   sigma_fission_barn: 582.6,  sigma_scatter_barn: 14.30,  sigma_total_barn:  695.71,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "U",   mass_number: 238, sigma_capture_barn:  2.717,  sigma_fission_barn: 0.000005,sigma_scatter_barn: 9.36,  sigma_total_barn:   12.08,  neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Pu",  mass_number: 239, sigma_capture_barn: 269.3,   sigma_fission_barn: 748.1,  sigma_scatter_barn:  7.7,   sigma_total_barn: 1025.1,   neutron_energy_ev: 0.0253 },
    NeutronCrossSection { isotope: "Pu",  mass_number: 241, sigma_capture_barn: 358.2,   sigma_fission_barn:1011.1,  sigma_scatter_barn: 11.0,   sigma_total_barn: 1380.3,   neutron_energy_ev: 0.0253 },
];

pub fn by_isotope(isotope: &str, mass_number: u32) -> Option<&'static NeutronCrossSection> {
    TABLE.iter().find(|n| n.isotope == isotope && n.mass_number == mass_number)
}
