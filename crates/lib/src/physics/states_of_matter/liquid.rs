#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiquidFamily {
    Water,
    Oil,
    Coolant,
    Fuel,
    Hydraulic,
}

#[derive(Debug, Clone, Copy)]
pub struct Liquid {
    pub name: &'static str,
    pub formula: &'static str,
    pub family: LiquidFamily,
    pub density_kg_m3_ref: f64,
    pub dynamic_viscosity_pa_s_ref: f64,
    pub bulk_modulus_pa: f64,
    pub specific_heat_j_kgk: f64,
    pub thermal_conductivity_w_mk: f64,
    pub surface_tension_n_m: f64,
    pub vapor_pressure_pa_ref: f64,
    pub temperature_ref_k: f64,
    pub viscosity_index: f64,
    pub pour_point_k: f64,
    pub flash_point_k: f64,
    pub shear_stability_index: f64,
    pub friction_coefficient: f64,
}

impl Liquid {
    pub fn kinematic_viscosity_m2_s(&self) -> f64 {
        self.dynamic_viscosity_pa_s_ref / self.density_kg_m3_ref
    }

    pub fn speed_of_sound_m_s(&self) -> f64 {
        (self.bulk_modulus_pa / self.density_kg_m3_ref).sqrt()
    }

    pub fn reynolds_number(&self, velocity_m_s: f64, length_m: f64) -> f64 {
        self.density_kg_m3_ref * velocity_m_s * length_m / self.dynamic_viscosity_pa_s_ref
    }

    pub fn prandtl_number(&self) -> f64 {
        self.dynamic_viscosity_pa_s_ref * self.specific_heat_j_kgk / self.thermal_conductivity_w_mk
    }

    pub fn from_molecule(
        molecule: &crate::constants::chemistry::molecules::Molecule,
        family: LiquidFamily,
    ) -> Option<Self> {
        use crate::constants::chemistry::thermo::{antoine, compressibility, heat_capacity, surface_tension};
        use crate::constants::physics::transport::{liquid_viscosity, thermal_conductivity};

        let density_g_cm3 = molecule.density_g_cm3?;
        let molar_mass_kg_per_mol = molecule.molar_mass * 1.0e-3;
        let cp = heat_capacity::by_formula_phase(molecule.formula, "liquid")?;
        let mu = liquid_viscosity::by_formula(molecule.formula)?;
        let k = thermal_conductivity::by_formula_phase(molecule.formula, "liquid")?;
        let st = surface_tension::by_formula(molecule.formula)?;

        let density_kg_m3 = density_g_cm3 * 1000.0;
        let cp_mass = cp.cp_j_molk_298 / molar_mass_kg_per_mol;
        let bulk_modulus_pa = compressibility::by_formula(molecule.formula)
            .map(|c| 1.0 / c.kappa_t_per_pa)
            .unwrap_or(2.2e9);
        let vapor_pressure = antoine::by_formula(molecule.formula)
            .map(|a| a.vapor_pressure_pa(crate::constants::physics::T_STD_CHEM))
            .unwrap_or(0.0);

        Some(Liquid {
            name: molecule.name,
            formula: molecule.formula,
            family,
            density_kg_m3_ref: density_kg_m3,
            dynamic_viscosity_pa_s_ref: mu.mu_pa_s,
            bulk_modulus_pa,
            specific_heat_j_kgk: cp_mass,
            thermal_conductivity_w_mk: k.k_w_per_mk,
            surface_tension_n_m: st.sigma_n_per_m,
            vapor_pressure_pa_ref: vapor_pressure,
            temperature_ref_k: crate::constants::physics::T_STD_CHEM,
            viscosity_index: 0.0,
            pour_point_k: 0.0,
            flash_point_k: 0.0,
            shear_stability_index: 0.0,
            friction_coefficient: 0.0,
        })
    }
}
