use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct Polymer {
    pub name: &'static str,
    pub abbreviation: &'static str,
    pub monomer_formula: &'static str,
    pub monomer_molar_mass: f64,
    pub density_g_cm3: f64,
    pub glass_transition_k: Option<f64>,
    pub melting_point_k: Option<f64>,
    pub polymer_type: &'static str,
    pub category: &'static str,
}

pub mod abs;
pub mod bakelite;
pub mod epoxy;
pub mod kevlar;
pub mod nylon66;
pub mod pet;
pub mod pmma;
pub mod polycarbonate;
pub mod polyethylene;
pub mod polypropylene;
pub mod polystyrene;
pub mod polyurethane;
pub mod ptfe;
pub mod pvc;
pub mod silicone_pdms;

fn init_polymers() -> Vec<Polymer> {
    vec![
        polyethylene::polymer(),
        polypropylene::polymer(),
        polystyrene::polymer(),
        pvc::polymer(),
        ptfe::polymer(),
        pet::polymer(),
        pmma::polymer(),
        nylon66::polymer(),
        kevlar::polymer(),
        polycarbonate::polymer(),
        polyurethane::polymer(),
        abs::polymer(),
        silicone_pdms::polymer(),
        bakelite::polymer(),
        epoxy::polymer(),
    ]
}

fn polymers() -> &'static Vec<Polymer> {
    static POLYMERS: OnceLock<Vec<Polymer>> = OnceLock::new();
    POLYMERS.get_or_init(init_polymers)
}

pub fn all() -> &'static [Polymer] {
    polymers()
}

pub fn by_name(name: &str) -> Option<&'static Polymer> {
    polymers().iter().find(|p| p.name.eq_ignore_ascii_case(name))
}

pub fn by_abbreviation(abbr: &str) -> Option<&'static Polymer> {
    polymers().iter().find(|p| p.abbreviation.eq_ignore_ascii_case(abbr))
}
