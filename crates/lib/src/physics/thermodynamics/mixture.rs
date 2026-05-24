use crate::physics::states_of_matter::gas::Gas;

#[derive(Debug, Clone)]
pub struct Mixture {
    pub components: Vec<(Gas, f64)>,
}

impl Mixture {
    pub fn new(components: Vec<(Gas, f64)>) -> Self {
        let total: f64 = components.iter().map(|(_, x)| *x).sum();
        let normalized = if total > 0.0 {
            components.into_iter().map(|(g, x)| (g, x / total)).collect()
        } else {
            components
        };
        Mixture { components: normalized }
    }

    pub fn molar_mass_kg_per_mol(&self) -> f64 {
        self.components.iter().map(|(g, x)| x * g.molar_mass_kg_per_mol).sum()
    }

    pub fn mass_fractions(&self) -> Vec<f64> {
        let m_avg = self.molar_mass_kg_per_mol();
        self.components.iter().map(|(g, x)| x * g.molar_mass_kg_per_mol / m_avg).collect()
    }

    pub fn cp_j_kgk(&self) -> f64 {
        let mass_fractions = self.mass_fractions();
        self.components.iter().zip(mass_fractions.iter())
            .map(|((g, _), w)| w * g.cp_j_kgk_ref).sum()
    }

    pub fn cv_j_kgk(&self) -> f64 {
        let mass_fractions = self.mass_fractions();
        self.components.iter().zip(mass_fractions.iter())
            .map(|((g, _), w)| w * g.cv_j_kgk_ref).sum()
    }

    pub fn gamma(&self) -> f64 {
        self.cp_j_kgk() / self.cv_j_kgk()
    }

    pub fn partial_pressure_pa(&self, total_pressure_pa: f64, index: usize) -> f64 {
        total_pressure_pa * self.components[index].1
    }

    pub fn density_ideal_kg_m3(&self, pressure_pa: f64, temperature_k: f64) -> f64 {
        let m_avg = self.molar_mass_kg_per_mol();
        pressure_pa * m_avg / (crate::constants::R_GAS * temperature_k)
    }

    pub fn dynamic_viscosity_pa_s(&self, temperature_k: f64) -> f64 {
        let n = self.components.len();
        let mu: Vec<f64> = self.components.iter()
            .map(|(g, _)| g.dynamic_viscosity_pa_s(temperature_k)).collect();
        let m: Vec<f64> = self.components.iter().map(|(g, _)| g.molar_mass_kg_per_mol).collect();
        let x: Vec<f64> = self.components.iter().map(|(_, xi)| *xi).collect();

        let mut total = 0.0;
        for i in 0..n {
            let mut denom = 0.0;
            for j in 0..n {
                let phi_ij = (1.0 + (mu[i] / mu[j]).sqrt() * (m[j] / m[i]).powf(0.25)).powi(2)
                    / (8.0 * (1.0 + m[i] / m[j])).sqrt();
                denom += x[j] * phi_ij;
            }
            if denom > 0.0 {
                total += x[i] * mu[i] / denom;
            }
        }
        total
    }

    pub fn thermal_conductivity_w_mk(&self, temperature_k: f64, conductivities: &[f64]) -> f64 {
        let n = self.components.len();
        let mu: Vec<f64> = self.components.iter()
            .map(|(g, _)| g.dynamic_viscosity_pa_s(temperature_k)).collect();
        let m: Vec<f64> = self.components.iter().map(|(g, _)| g.molar_mass_kg_per_mol).collect();
        let x: Vec<f64> = self.components.iter().map(|(_, xi)| *xi).collect();

        let mut total = 0.0;
        for i in 0..n {
            let mut denom = 0.0;
            for j in 0..n {
                let phi_ij = (1.0 + (mu[i] / mu[j]).sqrt() * (m[j] / m[i]).powf(0.25)).powi(2)
                    / (8.0 * (1.0 + m[i] / m[j])).sqrt();
                denom += x[j] * phi_ij;
            }
            if denom > 0.0 {
                total += x[i] * conductivities[i] / denom;
            }
        }
        total
    }
}
