pub fn scaffold_degradation_bulk(m0: f64, k: f64, t: f64) -> f64 {
    m0 * (-k * t).exp()
}

pub fn scaffold_degradation_surface(m0: f64, rate: f64, surface_area: f64, t: f64) -> f64 {
    (m0 - rate * surface_area * t).max(0.0)
}

pub fn porosity(void_volume: f64, total_volume: f64) -> f64 {
    void_volume / total_volume
}

pub fn pore_interconnectivity(connected_pores: f64, total_pores: f64) -> f64 {
    connected_pores / total_pores
}

pub fn mechanical_stiffness_degrading(e0: f64, degradation_fraction: f64, n: f64) -> f64 {
    e0 * (1.0 - degradation_fraction).powf(n)
}

pub fn scaffold_swelling_ratio(wet_mass: f64, dry_mass: f64) -> f64 {
    (wet_mass - dry_mass) / dry_mass
}

pub fn cell_seeding_efficiency(attached: f64, seeded: f64) -> f64 {
    attached / seeded.max(1e-30)
}

pub fn nutrient_diffusion_depth(
    diffusivity: f64,
    consumption_rate: f64,
    surface_concentration: f64,
) -> f64 {
    (2.0 * diffusivity * surface_concentration / consumption_rate.max(1e-30)).sqrt()
}

pub fn construct_viability(viable_cells: f64, total_cells: f64) -> f64 {
    viable_cells / total_cells.max(1e-30)
}

pub fn bioreactor_shear_stress(
    flow_rate: f64,
    viscosity: f64,
    channel_height: f64,
    channel_width: f64,
) -> f64 {
    6.0 * viscosity * flow_rate / (channel_width * channel_height * channel_height)
}

pub fn fiber_diameter_electrospinning(
    voltage: f64,
    flow_rate: f64,
    concentration: f64,
    distance: f64,
) -> f64 {
    concentration.powf(0.5) * (flow_rate / voltage).powf(0.3) * distance.powf(-0.2)
}

pub fn scaffold_compressive_modulus(stress: f64, strain: f64) -> f64 {
    stress / strain.max(1e-30)
}

pub fn scaffold_tortuosity(path_length: f64, straight_distance: f64) -> f64 {
    path_length / straight_distance.max(1e-30)
}

pub fn specific_surface_area(surface_area: f64, volume: f64) -> f64 {
    surface_area / volume.max(1e-30)
}

pub fn kozeny_carman_permeability(porosity_frac: f64, pore_diameter: f64) -> f64 {
    let phi = porosity_frac;
    phi.powi(3) * pore_diameter.powi(2) / (180.0 * (1.0 - phi).powi(2))
}

pub fn hydrogel_crosslink_density(shear_modulus: f64, temperature: f64) -> f64 {
    shear_modulus / (crate::constants::R_GAS * temperature)
}

pub fn hydrogel_mesh_size(
    crosslink_density: f64,
    cn: f64,
    bond_length: f64,
    mr: f64,
    polymer_density: f64,
) -> f64 {
    let mc = 1.0 / crosslink_density.max(1e-30);
    let v2 = polymer_density;
    v2.powf(-1.0 / 3.0) * (cn * mc / mr).sqrt() * bond_length
}

pub fn scaffold_surface_energy(contact_angle_deg: f64, liquid_tension: f64) -> f64 {
    let theta = contact_angle_deg * std::f64::consts::PI / 180.0;
    liquid_tension * (1.0 + theta.cos()) / 2.0
}

pub fn drug_release_higuchi(k_h: f64, t: f64) -> f64 {
    k_h * t.sqrt()
}

pub fn drug_release_korsmeyer_peppas(k: f64, t: f64, n: f64) -> f64 {
    (k * t.powf(n)).min(1.0)
}

pub fn degradation_molecular_weight(mw0: f64, k: f64, t: f64) -> f64 {
    mw0 * (-k * t).exp()
}

pub fn autocatalytic_degradation(mw0: f64, k: f64, acid_conc: f64, t: f64) -> f64 {
    mw0 * (-k * (1.0 + acid_conc) * t).exp()
}
