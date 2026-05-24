pub mod gas;
pub mod liquid;
pub mod plasma;
pub mod solid;

use crate::constants::chemistry::molecules::Molecule;
use gas::{Gas, GasFamily};
use liquid::{Liquid, LiquidFamily};
use solid::{Solid, SolidFamily};

#[derive(Debug, Clone)]
pub enum Material {
    Gas(Gas),
    Liquid(Liquid),
    Solid(Solid),
}

pub fn from_molecule(molecule: &Molecule) -> Option<Material> {
    match molecule.state_at_stp {
        "gas" => Gas::from_molecule(molecule, GasFamily::Process).map(Material::Gas),
        "liquid" => Liquid::from_molecule(molecule, LiquidFamily::Water).map(Material::Liquid),
        "solid" => Solid::from_molecule(molecule, SolidFamily::Ceramic).map(Material::Solid),
        _ => None,
    }
}

pub fn from_formula(formula: &str) -> Option<Material> {
    from_molecule(crate::constants::chemistry::molecules::by_formula(formula)?)
}
