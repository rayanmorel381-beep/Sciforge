use crate::moleculars::Gas;
use sciforge_hub::prelude::constants::physics::electrodynamics::dielectric;
use sciforge_hub::prelude::physics::electrodynamics::waves as sf_waves;

impl Gas {
    pub fn relative_permittivity(&self) -> Option<f64> {
        dielectric::by_formula(self.formula).map(|d| d.relative_permittivity)
    }

    pub fn em_phase_velocity_m_s(&self) -> Option<f64> {
        self.relative_permittivity()
            .map(|eps_r| sf_waves::phase_velocity(eps_r, 1.0))
    }

    pub fn em_refractive_index(&self) -> Option<f64> {
        self.relative_permittivity().map(|eps_r| eps_r.sqrt())
    }
}
