use super::aspiration::Aspiration;
use super::engine::Engine;
use super::bloc::Bloc;
use super::fuel::Fuel;
use super::layout::Layout;
use super::super::parts::{
    camshafts::Camshaft,
    compressors::Supercharger,
    exhausts::ExhaustSystem,
    flywheels::Flywheel,
    fuel_systems::FuelSystem,
    intake::IntakeSystem,
    pistons::PistonSet,
    turbos::Turbocharger,
    valves::{ValveTrain, VvtSystem},
};

#[derive(Debug, Clone, Copy)]
pub enum PistonKind { Cast, Forged, HighPerformance }

#[derive(Debug, Clone, Copy)]
pub enum ValveKind {
    TwoValveOhv,
    FourValveDohc(VvtSystem),
    FiveValve,
}

#[derive(Debug, Clone, Copy)]
pub enum CamKind {
    Ohv,
    Sohc { vtec: bool },
    Dohc { vvti: bool },
}

#[derive(Debug, Clone, Copy)]
pub enum IntakeKind { Na, Turbo(u8), TwinCharged(u8), Supercharged }

#[derive(Debug, Clone, Copy)]
pub enum FuelKind {
    Carbureted,
    Port { injector_flow_cc_min: f64 },
    Direct { injector_flow_cc_min: f64 },
    Dual { primary_flow_cc_min: f64, secondary_flow_cc_min: f64 },
}

#[derive(Debug, Clone, Copy)]
pub enum ExhaustKind { Stock, Performance, Race }

#[derive(Debug, Clone, Copy)]
pub enum FlywheelKind { SingleMass, DualMass, Lightweight }

#[derive(Debug, Clone, Copy)]
pub enum SuperchargerKind { Roots, TwinScrew { intercooled: bool }, Centrifugal }

#[derive(Debug, Clone)]
pub struct HomemadeEngine {
    layout: Option<Layout>,
    cylinders: Option<u8>,
    bore_mm: Option<f64>,
    stroke_mm: Option<f64>,
    bank_angle_deg: Option<f64>,
    compression_ratio: Option<f64>,
    aspiration: Option<Aspiration>,
    max_boost_bar: f64,
    conrod_ratio_lambda: f64,
    design_safety_factor: f64,
    variant: &'static str,

    piston_kind: Option<PistonKind>,
    valve_kind: Option<ValveKind>,
    cam_kind: Option<CamKind>,
    intake_kind: Option<IntakeKind>,
    fuel_kind: Option<FuelKind>,
    exhaust_kind: Option<ExhaustKind>,
    flywheel_kind: Option<FlywheelKind>,
    supercharger_kind: Option<SuperchargerKind>,
    supercharger_displacement_cc: Option<u32>,

    fuel_type: Fuel,

    turbo_inducer_mm: Option<f64>,
    turbo_exducer_mm: Option<f64>,

    flywheel_mass_kg: Option<f64>,
    flywheel_diameter_mm: Option<f64>,

    override_rpm_max: Option<f64>,
    override_eta_thermal: Option<f64>,
    override_bsfc: Option<f64>,
    override_power_kw: Option<f64>,
    override_knock_factor: Option<f64>,
    override_egt_k: Option<f64>,
    override_charge_temp_k: Option<f64>,
    override_eta_compressor: Option<f64>,
    override_fatigue_life_hours: Option<f64>,
}

impl Default for HomemadeEngine {
    fn default() -> Self { Self::new() }
}

impl HomemadeEngine {
    pub fn new() -> Self {
        Self {
            layout: None,
            cylinders: None,
            bore_mm: None,
            stroke_mm: None,
            bank_angle_deg: None,
            compression_ratio: None,
            aspiration: None,
            max_boost_bar: 0.0,
            conrod_ratio_lambda: 0.30,
            design_safety_factor: 2.5,
            variant: "",
            piston_kind: None,
            valve_kind: None,
            cam_kind: None,
            intake_kind: None,
            fuel_kind: None,
            exhaust_kind: None,
            flywheel_kind: None,
            supercharger_kind: None,
            supercharger_displacement_cc: None,
            fuel_type: Fuel::Gasoline,
            turbo_inducer_mm: None,
            turbo_exducer_mm: None,
            flywheel_mass_kg: None,
            flywheel_diameter_mm: None,
            override_rpm_max: None,
            override_eta_thermal: None,
            override_bsfc: None,
            override_power_kw: None,
            override_knock_factor: None,
            override_egt_k: None,
            override_charge_temp_k: None,
            override_eta_compressor: None,
            override_fatigue_life_hours: None,
        }
    }

    pub fn layout(mut self, l: Layout) -> Self { self.layout = Some(l); self }
    pub fn cylinders(mut self, n: u8) -> Self { self.cylinders = Some(n); self }
    pub fn bore_mm(mut self, b: f64) -> Self { self.bore_mm = Some(b); self }
    pub fn stroke_mm(mut self, s: f64) -> Self { self.stroke_mm = Some(s); self }
    pub fn bank_angle_deg(mut self, a: f64) -> Self { self.bank_angle_deg = Some(a); self }
    pub fn compression_ratio(mut self, cr: f64) -> Self { self.compression_ratio = Some(cr); self }
    pub fn aspiration(mut self, a: Aspiration) -> Self { self.aspiration = Some(a); self }
    pub fn max_boost_bar(mut self, b: f64) -> Self { self.max_boost_bar = b; self }
    pub fn conrod_ratio_lambda(mut self, l: f64) -> Self { self.conrod_ratio_lambda = l; self }
    pub fn design_safety_factor(mut self, f: f64) -> Self { self.design_safety_factor = f; self }
    pub fn variant(mut self, v: &'static str) -> Self { self.variant = v; self }

    pub fn pistons(mut self, k: PistonKind) -> Self { self.piston_kind = Some(k); self }
    pub fn valves(mut self, k: ValveKind) -> Self { self.valve_kind = Some(k); self }
    pub fn camshaft(mut self, k: CamKind) -> Self { self.cam_kind = Some(k); self }
    pub fn intake(mut self, k: IntakeKind) -> Self { self.intake_kind = Some(k); self }
    pub fn fuel(mut self, k: FuelKind) -> Self { self.fuel_kind = Some(k); self }
    pub fn fuel_type(mut self, f: Fuel) -> Self { self.fuel_type = f; self }
    pub fn exhaust(mut self, k: ExhaustKind) -> Self { self.exhaust_kind = Some(k); self }
    pub fn flywheel(mut self, k: FlywheelKind) -> Self { self.flywheel_kind = Some(k); self }
    pub fn flywheel_mass_kg(mut self, m: f64) -> Self { self.flywheel_mass_kg = Some(m); self }
    pub fn flywheel_diameter_mm(mut self, d: f64) -> Self { self.flywheel_diameter_mm = Some(d); self }

    pub fn turbo_dimensions_mm(mut self, inducer: f64, exducer: f64) -> Self {
        self.turbo_inducer_mm = Some(inducer);
        self.turbo_exducer_mm = Some(exducer);
        self
    }

    pub fn supercharger(mut self, k: SuperchargerKind, displacement_cc: u32) -> Self {
        self.supercharger_kind = Some(k);
        self.supercharger_displacement_cc = Some(displacement_cc);
        self
    }

    pub fn override_rpm_max(mut self, v: f64) -> Self { self.override_rpm_max = Some(v); self }
    pub fn override_eta_thermal(mut self, v: f64) -> Self { self.override_eta_thermal = Some(v); self }
    pub fn override_bsfc_g_per_kwh(mut self, v: f64) -> Self { self.override_bsfc = Some(v); self }
    pub fn override_power_kw(mut self, v: f64) -> Self { self.override_power_kw = Some(v); self }
    pub fn override_knock_factor(mut self, v: f64) -> Self { self.override_knock_factor = Some(v); self }
    pub fn override_egt_k(mut self, v: f64) -> Self { self.override_egt_k = Some(v); self }
    pub fn override_charge_temp_k(mut self, v: f64) -> Self { self.override_charge_temp_k = Some(v); self }
    pub fn override_eta_compressor(mut self, v: f64) -> Self { self.override_eta_compressor = Some(v); self }
    pub fn override_fatigue_life_hours(mut self, v: f64) -> Self { self.override_fatigue_life_hours = Some(v); self }

    pub fn build(self) -> Result<Engine, &'static str> {
        let layout = self.layout.ok_or("layout requis")?;
        let cyl = self.cylinders.ok_or("cylinders requis")?;
        let bore = self.bore_mm.ok_or("bore_mm requis")?;
        let stroke = self.stroke_mm.ok_or("stroke_mm requis")?;
        let cr = self.compression_ratio.ok_or("compression_ratio requis")?;
        let aspiration = self.aspiration.ok_or("aspiration requise")?;

        let bank_angle = match layout {
            Layout::V | Layout::Vr | Layout::W => self
                .bank_angle_deg
                .ok_or("bank_angle_deg requis pour V/Vr/W")?,
            Layout::Flat => 180.0,
            _ => 0.0,
        };

        let displacement_cc = ((std::f64::consts::PI * bore.powi(2) / 4.0) * stroke * cyl as f64) as u32;
        let bloc = Bloc {
            layout,
            cylinders: cyl,
            displacement_cc,
            bore_mm: bore,
            stroke_mm: stroke,
            aspiration,
            variant: self.variant,
            compression_ratio: cr,
            max_boost_bar: self.max_boost_bar,
            conrod_ratio_lambda: self.conrod_ratio_lambda,
            design_safety_factor: self.design_safety_factor,
            head_variants: vec![super::super::parts::culasses::Culasse::dohc_4v_1p_pent_roof(cyl)],
        };

        let mut engine = super::engine::Engine::assemble_with_fuel(&bloc, self.fuel_type);
        engine.bank_angle_deg = bank_angle;

        if let Some(kind) = self.piston_kind {
            engine.pistons = match kind {
                PistonKind::Cast => PistonSet::cast(cyl, bore, stroke),
                PistonKind::Forged => PistonSet::forged(cyl, bore, stroke),
                PistonKind::HighPerformance => PistonSet::high_performance(cyl, bore, stroke),
            };
        }

        if let Some(kind) = self.valve_kind {
            engine.valves = match kind {
                ValveKind::TwoValveOhv => ValveTrain::two_valve_ohv(cyl),
                ValveKind::FourValveDohc(vvt) => ValveTrain::four_valve_dohc(cyl, vvt),
                ValveKind::FiveValve => ValveTrain::five_valve(cyl),
            };
        }

        if let Some(kind) = self.cam_kind {
            engine.camshaft = match kind {
                CamKind::Ohv => Camshaft::ohv(cyl as u16),
                CamKind::Sohc { vtec } => Camshaft::sohc(cyl as u16, vtec),
                CamKind::Dohc { vvti } => Camshaft::dohc(cyl as u16, vvti),
            };
        }

        let disp_l = bloc.displacement_l();

        if let Some(kind) = self.intake_kind {
            engine.intake = match kind {
                IntakeKind::Na => IntakeSystem::naturally_aspirated(disp_l),
                IntakeKind::Turbo(1) => IntakeSystem::turbocharged(disp_l),
                IntakeKind::Turbo(2) => IntakeSystem::twin_turbo(disp_l),
                IntakeKind::Turbo(_) => IntakeSystem::multi_turbo(disp_l),
                IntakeKind::TwinCharged(_) => IntakeSystem::twincharged(disp_l),
                IntakeKind::Supercharged => IntakeSystem::supercharged(disp_l),
            };
        }

        if let (Some(inducer), Some(exducer)) = (self.turbo_inducer_mm, self.turbo_exducer_mm) {
            engine.turbo = Some(Turbocharger {
                compressor_inducer_mm: inducer,
                turbine_exducer_mm: exducer,
                max_boost_bar: self.max_boost_bar,
            });
        }

        if let (Some(kind), Some(disp_cc)) = (self.supercharger_kind, self.supercharger_displacement_cc) {
            let mut sc = match kind {
                SuperchargerKind::Roots => Supercharger::roots(disp_cc),
                SuperchargerKind::TwinScrew { intercooled } => Supercharger::twin_screw(disp_cc, intercooled),
                SuperchargerKind::Centrifugal => Supercharger::centrifugal(disp_cc),
            };
            sc.max_boost_bar = self.max_boost_bar;
            engine.supercharger = Some(sc);
        }

        if let Some(kind) = self.fuel_kind {
            engine.fuel = match kind {
                FuelKind::Carbureted => FuelSystem::carbureted(cyl),
                FuelKind::Port { injector_flow_cc_min } => FuelSystem::port_injection(cyl, injector_flow_cc_min),
                FuelKind::Direct { injector_flow_cc_min } => FuelSystem::direct_injection(cyl, injector_flow_cc_min),
                FuelKind::Dual { primary_flow_cc_min, secondary_flow_cc_min } => {
                    FuelSystem::dual_injection(cyl, primary_flow_cc_min, secondary_flow_cc_min)
                }
            };
        }

        if let Some(kind) = self.exhaust_kind {
            engine.exhaust = match kind {
                ExhaustKind::Stock => ExhaustSystem::stock(cyl),
                ExhaustKind::Performance => ExhaustSystem::performance(cyl),
                ExhaustKind::Race => ExhaustSystem::race(cyl),
            };
        }

        if let Some(kind) = self.flywheel_kind {
            let mass = self.flywheel_mass_kg.unwrap_or(5.0 + disp_l * 3.0);
            let dia = self.flywheel_diameter_mm.unwrap_or(220.0 + cyl as f64 * 4.0);
            engine.flywheel = match kind {
                FlywheelKind::SingleMass => Flywheel::single_mass(mass, dia),
                FlywheelKind::DualMass => Flywheel::dual_mass(mass, dia),
                FlywheelKind::Lightweight => Flywheel::lightweight(mass, dia),
            };
        }

        if let Some(v) = self.override_rpm_max { engine.rpm_max = v; }
        if let Some(v) = self.override_eta_thermal { engine.eta_thermal = v; }
        if let Some(v) = self.override_bsfc { engine.bsfc_g_per_kwh = v; }
        if let Some(v) = self.override_power_kw { engine.estimated_power_kw = v; }
        if let Some(v) = self.override_knock_factor { engine.knock_factor = v; }
        if let Some(v) = self.override_egt_k { engine.egt_k = v; }
        if let Some(v) = self.override_charge_temp_k { engine.charge_temp_k = v; }
        if let Some(v) = self.override_eta_compressor { engine.eta_compressor = v; }
        if let Some(v) = self.override_fatigue_life_hours { engine.fatigue_life_hours = v; }

        Ok(engine)
    }
}
