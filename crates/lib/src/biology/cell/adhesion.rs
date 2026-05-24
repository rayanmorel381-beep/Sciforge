pub fn cell_adhesion_energy(contact_area: f64, cadherin_density: f64, bond_energy: f64) -> f64 {
    contact_area * cadherin_density * bond_energy
}

pub fn integrin_focal_adhesion_force(integrin_count: f64, force_per_integrin: f64) -> f64 {
    integrin_count * force_per_integrin
}

pub fn adhesion_receptor_binding(ligand: f64, receptor: f64, kd: f64) -> f64 {
    ligand * receptor / (kd + ligand)
}

pub fn cell_cell_junction_strength(
    tight_junction: f64,
    adherens_junction: f64,
    desmosome: f64,
) -> f64 {
    tight_junction + adherens_junction + desmosome
}

pub fn chemotaxis_velocity(gradient: f64, sensitivity: f64, max_speed: f64) -> f64 {
    max_speed * gradient / (sensitivity + gradient)
}

pub fn haptotaxis_velocity(ecm_gradient: f64, adhesion_strength: f64, drag: f64) -> f64 {
    adhesion_strength * ecm_gradient / drag.max(1e-30)
}

pub fn durotaxis_force(stiffness_gradient: f64, cell_contractility: f64) -> f64 {
    cell_contractility * stiffness_gradient
}

pub fn collective_migration_speed(leader_speed: f64, follower_count: usize, coupling: f64) -> f64 {
    leader_speed / (1.0 + coupling * follower_count as f64)
}

pub fn wound_healing_rate(gap_width: f64, migration_speed: f64, proliferation_rate: f64) -> f64 {
    2.0 * migration_speed + proliferation_rate * gap_width
}

pub fn ecm_remodeling_rate(mmp_activity: f64, timp_activity: f64, substrate: f64) -> f64 {
    let net_activity = (mmp_activity - timp_activity).max(0.0);
    net_activity * substrate
}

pub fn cell_spreading_area(adhesion_strength: f64, cortical_tension: f64) -> f64 {
    std::f64::consts::PI * (adhesion_strength / cortical_tension.max(1e-30)).powi(2)
}

pub fn catch_bond_lifetime(force: f64, k0: f64, x1: f64, x2: f64) -> f64 {
    use crate::constants::K_B;
    let kbt = K_B * 310.0;
    1.0 / (k0 * (-force * x1 / kbt).exp() + k0 * (force * x2 / kbt).exp())
}
