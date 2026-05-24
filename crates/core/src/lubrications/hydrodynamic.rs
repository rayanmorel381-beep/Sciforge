use crate::lubrications::Grease;
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LubricationRegime {
    Boundary,
    Mixed,
    Hydrodynamic,
    Elastohydrodynamic,
}

pub fn petroff_friction_coefficient(
    dynamic_viscosity_pa_s: f64,
    rotational_speed_rev_per_s: f64,
    journal_radius_m: f64,
    radial_clearance_m: f64,
    bearing_pressure_pa: f64,
) -> f64 {
    2.0 * PI * PI * dynamic_viscosity_pa_s * rotational_speed_rev_per_s * journal_radius_m
        / (radial_clearance_m * bearing_pressure_pa)
}

pub fn sommerfeld_number(
    journal_radius_m: f64,
    radial_clearance_m: f64,
    dynamic_viscosity_pa_s: f64,
    rotational_speed_rev_per_s: f64,
    bearing_pressure_pa: f64,
) -> f64 {
    (journal_radius_m / radial_clearance_m).powi(2) * dynamic_viscosity_pa_s
        * rotational_speed_rev_per_s
        / bearing_pressure_pa
}

pub fn stribeck_parameter(
    dynamic_viscosity_pa_s: f64,
    sliding_velocity_m_s: f64,
    contact_pressure_pa: f64,
) -> f64 {
    dynamic_viscosity_pa_s * sliding_velocity_m_s / contact_pressure_pa
}

pub fn stribeck_regime(stribeck_param: f64) -> LubricationRegime {
    if stribeck_param < 1.0e-9 {
        LubricationRegime::Boundary
    } else if stribeck_param < 1.0e-7 {
        LubricationRegime::Mixed
    } else if stribeck_param < 1.0e-5 {
        LubricationRegime::Elastohydrodynamic
    } else {
        LubricationRegime::Hydrodynamic
    }
}

pub fn reynolds_thin_film(
    density_kg_m3: f64,
    velocity_m_s: f64,
    film_thickness_m: f64,
    dynamic_viscosity_pa_s: f64,
) -> f64 {
    density_kg_m3 * velocity_m_s * film_thickness_m / dynamic_viscosity_pa_s
}

#[derive(Debug, Clone, Copy)]
pub struct JournalBearing {
    pub grease: Grease,
    pub journal_radius_m: f64,
    pub bearing_length_m: f64,
    pub radial_clearance_m: f64,
    pub rotational_speed_rev_per_s: f64,
    pub radial_load_n: f64,
}

impl JournalBearing {
    pub fn projected_area_m2(&self) -> f64 {
        2.0 * self.journal_radius_m * self.bearing_length_m
    }

    pub fn bearing_pressure_pa(&self) -> f64 {
        self.radial_load_n / self.projected_area_m2()
    }

    pub fn surface_velocity_m_s(&self) -> f64 {
        2.0 * PI * self.journal_radius_m * self.rotational_speed_rev_per_s
    }

    pub fn sommerfeld(&self) -> f64 {
        sommerfeld_number(
            self.journal_radius_m,
            self.radial_clearance_m,
            self.grease.dynamic_viscosity_pa_s(),
            self.rotational_speed_rev_per_s,
            self.bearing_pressure_pa(),
        )
    }

    pub fn petroff_friction_coefficient(&self) -> f64 {
        petroff_friction_coefficient(
            self.grease.dynamic_viscosity_pa_s(),
            self.rotational_speed_rev_per_s,
            self.journal_radius_m,
            self.radial_clearance_m,
            self.bearing_pressure_pa(),
        )
    }

    pub fn petroff_friction_torque_n_m(&self) -> f64 {
        4.0 * PI * PI * self.grease.dynamic_viscosity_pa_s() * self.rotational_speed_rev_per_s
            * self.journal_radius_m.powi(3)
            * self.bearing_length_m
            / self.radial_clearance_m
    }

    pub fn power_loss_w(&self) -> f64 {
        self.petroff_friction_torque_n_m() * 2.0 * PI * self.rotational_speed_rev_per_s
    }

    pub fn lubrication_regime(&self) -> LubricationRegime {
        stribeck_regime(stribeck_parameter(
            self.grease.dynamic_viscosity_pa_s(),
            self.surface_velocity_m_s(),
            self.bearing_pressure_pa(),
        ))
    }
}
