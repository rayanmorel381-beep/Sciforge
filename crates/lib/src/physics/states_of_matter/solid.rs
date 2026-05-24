use crate::constants::physics::elements::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolidFamily {
    Aluminum,
    Iron,
    Titanium,
    Nickel,
    Magnesium,
    Copper,
    Composite,
    Plastic,
    Ceramic,
    Glass,
}

#[derive(Debug, Clone, Copy)]
pub struct Solid {
    pub element: Option<&'static Element>,
    pub name: &'static str,
    pub formula: &'static str,
    pub family: SolidFamily,
    pub density_kg_m3: f64,
    pub young_modulus_pa: f64,
    pub poisson_ratio: f64,
    pub yield_strength_pa: f64,
    pub ultimate_strength_pa: f64,
    pub thermal_conductivity_w_mk: f64,
    pub thermal_expansion_per_k: f64,
    pub specific_heat_j_kgk: f64,
    pub max_service_temp_k: f64,
    pub fatigue_strength_coeff_pa: f64,
    pub fatigue_strength_exponent: f64,
    pub hardness_hv: f64,
    pub cost_eur_kg: f64,
}

impl Solid {
    pub fn density(&self) -> f64 {
        self.density_kg_m3
    }

    pub fn shear_modulus_pa(&self) -> f64 {
        self.young_modulus_pa / (2.0 * (1.0 + self.poisson_ratio))
    }

    pub fn bulk_modulus_pa(&self) -> f64 {
        self.young_modulus_pa / (3.0 * (1.0 - 2.0 * self.poisson_ratio))
    }

    pub fn longitudinal_wave_speed_m_s(&self) -> f64 {
        let nu = self.poisson_ratio;
        let factor = (1.0 - nu) / ((1.0 + nu) * (1.0 - 2.0 * nu));
        (self.young_modulus_pa * factor / self.density_kg_m3).sqrt()
    }

    pub fn shear_wave_speed_m_s(&self) -> f64 {
        (self.shear_modulus_pa() / self.density_kg_m3).sqrt()
    }

    pub fn fatigue_strength_pa(&self, cycles: f64) -> f64 {
        self.fatigue_strength_coeff_pa * cycles.powf(self.fatigue_strength_exponent)
    }

    pub fn from_molecule(
        molecule: &crate::constants::chemistry::molecules::Molecule,
        family: SolidFamily,
    ) -> Option<Self> {
        use crate::constants::chemistry::thermo::heat_capacity;
        use crate::constants::physics::elements;
        use crate::constants::physics::solid_mechanics::{hardness, yield_strength};
        use crate::constants::physics::transport::{thermal_conductivity, thermal_expansion};

        let density_g_cm3 = molecule.density_g_cm3?;
        let molar_mass_kg_per_mol = molecule.molar_mass * 1.0e-3;
        let cp = heat_capacity::by_formula_phase(molecule.formula, "solid")?;
        let k = thermal_conductivity::by_formula_phase(molecule.formula, "solid")?;
        let alpha = thermal_expansion::by_formula_phase(molecule.formula, "solid")?;

        let density_kg_m3 = density_g_cm3 * 1000.0;
        let cp_mass = cp.cp_j_molk_298 / molar_mass_kg_per_mol;
        let element = elements::by_symbol(molecule.formula);
        let yld = yield_strength::by_material(molecule.formula);
        let hv = hardness::by_material(molecule.formula).and_then(|h| h.vickers_hv);
        let max_t = molecule.melting_point_k.unwrap_or(0.0);

        Some(Solid {
            element,
            name: molecule.name,
            formula: molecule.formula,
            family,
            density_kg_m3,
            young_modulus_pa: 0.0,
            poisson_ratio: 0.30,
            yield_strength_pa: yld.map(|y| y.yield_pa).unwrap_or(0.0),
            ultimate_strength_pa: yld.map(|y| y.ultimate_pa).unwrap_or(0.0),
            thermal_conductivity_w_mk: k.k_w_per_mk,
            thermal_expansion_per_k: alpha.alpha_per_k,
            specific_heat_j_kgk: cp_mass,
            max_service_temp_k: max_t,
            fatigue_strength_coeff_pa: 0.0,
            fatigue_strength_exponent: 0.0,
            hardness_hv: hv.unwrap_or(0.0),
            cost_eur_kg: 0.0,
        })
    }
}
