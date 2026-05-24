use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct Molecule {
    pub formula: &'static str,
    pub name: &'static str,
    pub composition: &'static [(u32, u32)],
    pub molar_mass: f64,
    pub category: &'static str,
    pub state_at_stp: &'static str,
    pub melting_point_k: Option<f64>,
    pub boiling_point_k: Option<f64>,
    pub density_g_cm3: Option<f64>,
}

mod biomolecules;
mod hydrocarbons;
mod inorganic;
mod minerals;
mod organic;
mod pharmaceuticals;

fn init_molecules() -> Vec<Molecule> {
    let mut all = Vec::new();
    all.extend(inorganic::all());
    all.extend(hydrocarbons::all());
    all.extend(organic::all());
    all.extend(biomolecules::all());
    all.extend(minerals::all());
    all.extend(pharmaceuticals::all());
    all
}

fn molecules() -> &'static Vec<Molecule> {
    static MOLECULES: OnceLock<Vec<Molecule>> = OnceLock::new();
    MOLECULES.get_or_init(init_molecules)
}

pub fn all() -> &'static [Molecule] {
    molecules()
}

pub fn by_formula(formula: &str) -> Option<&'static Molecule> {
    molecules().iter().find(|m| m.formula == formula)
}

pub fn by_name(name: &str) -> Option<&'static Molecule> {
    molecules().iter().find(|m| m.name.eq_ignore_ascii_case(name))
}
