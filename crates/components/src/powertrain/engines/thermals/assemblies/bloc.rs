use super::aspiration::Aspiration;
use super::layout::Layout;
use super::super::parts;

#[derive(Debug, Clone)]
pub struct Bloc {
    pub layout: Layout,
    pub cylinders: u8,
    pub displacement_cc: u32,
    pub bore_mm: f64,
    pub stroke_mm: f64,
    pub aspiration: Aspiration,
    pub variant: &'static str,
    pub compression_ratio: f64,
    pub max_boost_bar: f64,
    pub conrod_ratio_lambda: f64,
    pub design_safety_factor: f64,
    pub head_variants: Vec<parts::culasses::Culasse>,
}

#[derive(Debug, Clone)]
pub struct Topology {
    pub cylinders: parts::cylinders::CylinderBank,
    pub crankshaft: parts::crankshafts::Crankshaft,
    pub camshaft: parts::camshafts::Camshaft,
    pub valves: parts::valves::ValveTrain,
    pub bank_angle_deg: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct BoostState {
    pub pressure_ratio: f64,
    pub eta_compressor: f64,
    pub t_charge_k: f64,
    pub charge_density_kg_m3: f64,
    pub runner_target_length_m: f64,
    pub runner_actual_length_m: f64,
    pub runner_tuning_factor: f64,
    pub eta_volumetric: f64,
}

#[derive(Debug, Clone)]
pub struct SelectedParts {
    pub pistons: parts::pistons::PistonSet,
    pub intake: parts::intake::IntakeSystem,
    pub turbo: Option<parts::turbos::Turbocharger>,
    pub supercharger: Option<parts::compressors::Supercharger>,
    pub fuel: parts::fuel_systems::FuelSystem,
    pub exhaust: parts::exhausts::ExhaustSystem,
    pub flywheel: parts::flywheels::Flywheel,
    pub timing: parts::distributions::TimingDrive,
}

impl Bloc {
    pub fn code(&self) -> String {
        let disp = (self.displacement_cc + 50) / 100;
        let base = format!(
            "{}{}-{:02}-{}",
            self.layout.code(),
            self.cylinders,
            disp,
            self.aspiration.code(),
        );
        if self.variant.is_empty() {
            base
        } else {
            format!("{}-{}", base, self.variant)
        }
    }

    pub fn displacement_l(&self) -> f64 {
        self.displacement_cc as f64 / 1000.0
    }

    pub fn bank_angle_deg(&self) -> f64 {
        match self.layout {
            Layout::V => {
                let even_fire = 720.0 / self.cylinders as f64;
                if even_fire > 90.0 { 90.0 } else { even_fire }
            }
            Layout::Vr => 15.0,
            Layout::W => 72.0,
            Layout::Flat => 180.0,
            _ => 0.0,
        }
    }

    pub fn mean_piston_speed_limit_m_s(&self) -> f64 {
        match self.layout {
            Layout::Radial => 12.0,
            _ => match self.aspiration {
                Aspiration::Na if self.compression_ratio >= 13.0 => 25.0,
                Aspiration::Turbo(_) | Aspiration::TwinCharged(_) | Aspiration::Supercharged => 22.0,
                _ => 23.0,
            },
        }
    }

    pub fn rpm_max(&self) -> f64 {
        let stroke_m = self.stroke_mm / 1000.0;
        (self.mean_piston_speed_limit_m_s() / (2.0 * stroke_m)) * 60.0
    }

    pub fn topology(&self) -> Topology {
        let cyl = self.cylinders;
        let bore = self.bore_mm;
        let stroke = self.stroke_mm;
        let bank_angle_deg = self.bank_angle_deg();

        let cylinders = match self.layout {
            Layout::Inline => parts::cylinders::CylinderBank::inline(cyl, bore, self.compression_ratio),
            Layout::V => parts::cylinders::CylinderBank::v(cyl, bore, self.compression_ratio),
            Layout::Vr => parts::cylinders::CylinderBank::v(cyl, bore, self.compression_ratio),
            Layout::W => parts::cylinders::CylinderBank::w(cyl, bore, self.compression_ratio),
            Layout::Flat => parts::cylinders::CylinderBank::flat(cyl, bore, self.compression_ratio),
            Layout::Radial => parts::cylinders::CylinderBank::radial(cyl, bore, self.compression_ratio),
        };

        let crankshaft = match self.layout {
            Layout::Inline => parts::crankshafts::Crankshaft::inline(cyl, stroke),
            Layout::V | Layout::Vr => parts::crankshafts::Crankshaft::v(cyl, bank_angle_deg, stroke),
            Layout::Flat => parts::crankshafts::Crankshaft::flat(cyl, stroke),
            Layout::W => parts::crankshafts::Crankshaft::w(cyl, stroke),
            Layout::Radial => parts::crankshafts::Crankshaft::radial(cyl, stroke),
        };

        let camshaft = if matches!(self.layout, Layout::Radial) {
            parts::camshafts::Camshaft::ohv(cyl as u16)
        } else {
            parts::camshafts::Camshaft::dohc(cyl as u16, true)
        };

        let valves = if matches!(self.layout, Layout::Radial) {
            parts::valves::ValveTrain::two_valve_ohv(cyl)
        } else {
            parts::valves::ValveTrain::four_valve_dohc(cyl, parts::valves::VvtSystem::DualVvt)
        };

        Topology { cylinders, crankshaft, camshaft, valves, bank_angle_deg }
    }

    pub fn boost(&self, rpm_max: f64) -> BoostState {
        let air_density_intake = 1.225;
        let t_ambient_k = 293.0;
        let p_ambient_pa = 101325.0;
        let r_specific_air = 287.05;
        let cp_air = 1005.0;
        let gamma_air = 1.40;
        let bore_m = self.bore_mm / 1000.0;
        let stroke_m = self.stroke_mm / 1000.0;
        let disp_l = self.displacement_l();
        let disp_m3 = disp_l / 1000.0;
        let pressure_ratio = 1.0 + self.max_boost_bar / 1.01325;

        let mean_piston_speed_m_s = 2.0 * stroke_m * rpm_max / 60.0;
        let m_dot_air_kg_s = air_density_intake * pressure_ratio * disp_m3 * (rpm_max / 120.0);

        let eta_compressor = if pressure_ratio <= 1.0 {
            1.0
        } else {
            let pr_optimum = match self.aspiration {
                Aspiration::Supercharged | Aspiration::TwinCharged(_) => 1.6,
                _ => 2.2,
            };
            let pr_penalty = 1.0 - 0.05 * (pressure_ratio - pr_optimum).powi(2);
            let flow_optimum_kg_s = 0.05 + disp_l * 0.04;
            let flow_penalty = 1.0
                - 0.5 * ((m_dot_air_kg_s - flow_optimum_kg_s) / flow_optimum_kg_s).powi(2);
            (0.82 * pr_penalty.max(0.4) * flow_penalty.max(0.4)).clamp(0.55, 0.82)
        };

        let t_after_compressor_k = t_ambient_k
            * (1.0 + (pressure_ratio.powf((gamma_air - 1.0) / gamma_air) - 1.0) / eta_compressor);

        let (ua_ref_w_per_k_per_l, t_coolant_k) = match self.aspiration {
            Aspiration::Na => (0.0, t_ambient_k),
            Aspiration::Supercharged => (350.0, t_ambient_k + 8.0),
            _ => (500.0, t_ambient_k + 8.0),
        };
        let ua_w_per_k = ua_ref_w_per_k_per_l * disp_l;
        let intercooler_eff = if m_dot_air_kg_s > 1.0e-6 && ua_w_per_k > 0.0 {
            let ntu = ua_w_per_k / (m_dot_air_kg_s * cp_air);
            1.0 - (-ntu).exp()
        } else {
            0.0
        };
        let t_charge_k = t_after_compressor_k
            - intercooler_eff * (t_after_compressor_k - t_coolant_k);
        let p_charge_pa = p_ambient_pa * pressure_ratio;
        let charge_density_kg_m3 = p_charge_pa / (r_specific_air * t_charge_k);

        let speed_of_sound_m_s = (gamma_air * r_specific_air * t_charge_k).sqrt();
        let intake_event_freq_hz = (rpm_max / 60.0) * 2.0;
        let runner_target_length_m = speed_of_sound_m_s / (4.0 * intake_event_freq_hz);
        let runner_actual_length_m = (bore_m * 4.0).clamp(0.15, 0.45);
        let resonance_detuning = (runner_actual_length_m / runner_target_length_m).ln();
        let runner_tuning_factor = 1.0
            + 0.10 * (-resonance_detuning.powi(2) * 4.0).exp()
            - 0.20 * (1.0 - (-resonance_detuning.powi(2) * 1.5).exp());

        let valve_curtain_area_m2 = std::f64::consts::PI
            * (bore_m * 0.45)
            * (bore_m * 0.45 * 0.25)
            * 2.0;
        let cylinder_swept_area_m2 = std::f64::consts::PI * bore_m.powi(2) / 4.0;
        let valve_gas_velocity_m_s =
            mean_piston_speed_m_s * cylinder_swept_area_m2 / valve_curtain_area_m2.max(1.0e-6);
        let mach_valve = valve_gas_velocity_m_s / speed_of_sound_m_s;
        let bernoulli_loss_coeff = 1.5;
        let intake_dp_pa = 0.5
            * charge_density_kg_m3
            * valve_gas_velocity_m_s.powi(2)
            * bernoulli_loss_coeff;
        let bernoulli_efficiency = (1.0 - intake_dp_pa / p_charge_pa).max(0.5);
        let mach_choke_penalty = if mach_valve > 0.5 {
            1.0 - (mach_valve - 0.5).powi(2) * 2.0
        } else {
            1.0
        }
        .max(0.4);

        let eta_volumetric_base: f64 = match self.aspiration {
            Aspiration::Na => 0.92,
            Aspiration::Turbo(_) | Aspiration::TwinCharged(_) => 0.95,
            Aspiration::Supercharged => 0.98,
        };
        let eta_volumetric = (eta_volumetric_base
            * runner_tuning_factor
            * bernoulli_efficiency
            * mach_choke_penalty)
            .clamp(0.40, 1.10);

        BoostState {
            pressure_ratio,
            eta_compressor,
            t_charge_k,
            charge_density_kg_m3,
            runner_target_length_m,
            runner_actual_length_m,
            runner_tuning_factor,
            eta_volumetric,
        }
    }

    pub fn select_parts(
        &self,
        power_kw: f64,
        von_mises_pa: f64,
        piston_crown_temp_k: f64,
        buckling_safety: f64,
        injector_flow_cc_min: f64,
        allowable_cast_alu_pa: f64,
        allowable_forged_alu_pa: f64,
        allowable_titanium_pa: f64,
        cast_alu_max_temp_k: f64,
        forged_alu_max_temp_k: f64,
    ) -> SelectedParts {
        let cyl = self.cylinders;
        let bore = self.bore_mm;
        let stroke = self.stroke_mm;
        let disp_l = self.displacement_l();

        let pistons = if piston_crown_temp_k > forged_alu_max_temp_k
            || von_mises_pa > allowable_titanium_pa
            || buckling_safety < 2.0
        {
            parts::pistons::PistonSet::high_performance(cyl, bore, stroke)
        } else if piston_crown_temp_k > cast_alu_max_temp_k
            || von_mises_pa > allowable_forged_alu_pa
            || von_mises_pa > allowable_cast_alu_pa
        {
            parts::pistons::PistonSet::forged(cyl, bore, stroke)
        } else {
            parts::pistons::PistonSet::cast(cyl, bore, stroke)
        };

        let (intake, turbo, supercharger) = match self.aspiration {
            Aspiration::Na => (parts::intake::IntakeSystem::naturally_aspirated(disp_l), None, None),
            Aspiration::Turbo(n) => {
                let inducer = 30.0 + (disp_l / n as f64) * 12.0 + self.max_boost_bar * 8.0;
                let t = parts::turbos::Turbocharger {
                    compressor_inducer_mm: inducer,
                    turbine_exducer_mm: inducer * 1.05,
                    max_boost_bar: self.max_boost_bar,
                };
                let intake = if n >= 3 {
                    parts::intake::IntakeSystem::multi_turbo(disp_l)
                } else if n == 2 {
                    parts::intake::IntakeSystem::twin_turbo(disp_l)
                } else {
                    parts::intake::IntakeSystem::turbocharged(disp_l)
                };
                (intake, Some(t), None)
            }
            Aspiration::TwinCharged(n) => {
                let inducer = 30.0 + (disp_l / n as f64) * 12.0 + self.max_boost_bar * 6.0;
                let t = parts::turbos::Turbocharger {
                    compressor_inducer_mm: inducer,
                    turbine_exducer_mm: inducer * 1.05,
                    max_boost_bar: self.max_boost_bar * 0.6,
                };
                let sc_disp_cc = (self.displacement_cc as f64 * 0.4) as u32;
                let mut s = parts::compressors::Supercharger::twin_screw(sc_disp_cc, true);
                s.max_boost_bar = self.max_boost_bar * 0.5;
                (parts::intake::IntakeSystem::twincharged(disp_l), Some(t), Some(s))
            }
            Aspiration::Supercharged => {
                let sc_disp_cc = (self.displacement_cc as f64 * 0.4) as u32;
                let mut s = parts::compressors::Supercharger::twin_screw(sc_disp_cc, true);
                s.max_boost_bar = self.max_boost_bar;
                (parts::intake::IntakeSystem::supercharged(disp_l), None, Some(s))
            }
        };

        let fuel = match self.aspiration {
            Aspiration::Na => parts::fuel_systems::FuelSystem::port_injection(cyl, injector_flow_cc_min),
            Aspiration::Turbo(n) if n >= 2 => parts::fuel_systems::FuelSystem::dual_injection(
                cyl,
                injector_flow_cc_min,
                injector_flow_cc_min,
            ),
            Aspiration::TwinCharged(_) => parts::fuel_systems::FuelSystem::dual_injection(
                cyl,
                injector_flow_cc_min,
                injector_flow_cc_min,
            ),
            _ => parts::fuel_systems::FuelSystem::direct_injection(cyl, injector_flow_cc_min),
        };

        let exhaust = if power_kw >= 200.0 {
            parts::exhausts::ExhaustSystem::race(cyl)
        } else if self.max_boost_bar > 0.0 || self.compression_ratio >= 12.0 || power_kw >= 110.0 {
            parts::exhausts::ExhaustSystem::performance(cyl)
        } else {
            parts::exhausts::ExhaustSystem::stock(cyl)
        };

        let flywheel_mass = 5.0 + disp_l * 3.0;
        let flywheel_diameter = 220.0 + (cyl as f64) * 4.0;
        let flywheel = if power_kw >= 150.0 || self.max_boost_bar >= 1.2 {
            parts::flywheels::Flywheel::dual_mass(flywheel_mass, flywheel_diameter)
        } else if self.compression_ratio >= 12.0 {
            parts::flywheels::Flywheel::lightweight(flywheel_mass * 0.6, flywheel_diameter)
        } else {
            parts::flywheels::Flywheel::single_mass(flywheel_mass, flywheel_diameter)
        };

        let timing = match self.layout {
            Layout::Radial => parts::distributions::TimingDrive::gear_drive(),
            Layout::V | Layout::W | Layout::Vr => parts::distributions::TimingDrive::dual_chain(),
            _ => if cyl >= 6 {
                parts::distributions::TimingDrive::chain()
            } else {
                parts::distributions::TimingDrive::belt()
            },
        };

        SelectedParts { pistons, intake, turbo, supercharger, fuel, exhaust, flywheel, timing }
    }
}
