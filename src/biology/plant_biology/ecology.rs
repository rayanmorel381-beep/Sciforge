pub fn competitive_exclusion_tilman(r_star_a: f64, r_star_b: f64) -> &'static str {
    if r_star_a < r_star_b {
        "species A wins"
    } else if r_star_b < r_star_a {
        "species B wins"
    } else {
        "coexistence"
    }
}

pub fn allelopathy_effect(allelochemical_conc: f64, ic50: f64, max_inhibition: f64) -> f64 {
    max_inhibition * allelochemical_conc / (ic50 + allelochemical_conc)
}

pub fn light_competition_beer_lambert(light_above: f64, lai: f64, extinction_coeff: f64) -> f64 {
    light_above * (-extinction_coeff * lai).exp()
}

pub fn canopy_lai(leaf_area: f64, ground_area: f64) -> f64 {
    leaf_area / ground_area.max(1e-30)
}

pub fn sla(leaf_area: f64, leaf_dry_mass: f64) -> f64 {
    leaf_area / leaf_dry_mass.max(1e-30)
}

pub fn plant_defense_investment(growth_rate: f64, defense_allocation: f64) -> f64 {
    growth_rate * (1.0 - defense_allocation)
}

pub fn herbivory_damage(
    herbivore_density: f64,
    feeding_rate: f64,
    plant_biomass: f64,
    defense_level: f64,
) -> f64 {
    herbivore_density * feeding_rate * plant_biomass / (1.0 + defense_level)
}

pub fn seed_dispersal_kernel(distance: f64, mean_dispersal: f64) -> f64 {
    (1.0 / (2.0 * std::f64::consts::PI * mean_dispersal * mean_dispersal))
        * (-distance * distance / (2.0 * mean_dispersal * mean_dispersal)).exp()
}

pub fn pollination_success(pollinator_visits: f64, pollen_per_visit: f64, ovule_count: f64) -> f64 {
    let pollen_total = pollinator_visits * pollen_per_visit;
    (pollen_total / ovule_count).min(1.0)
}

pub fn nitrogen_fixation_symbiotic(
    nodule_mass: f64,
    nitrogenase_activity: f64,
    oxygen_limitation: f64,
) -> f64 {
    nodule_mass * nitrogenase_activity * (1.0 - oxygen_limitation)
}
