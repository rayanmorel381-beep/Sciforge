use super::Molecule;

pub mod alcohols;
pub mod aldehydes;
pub mod amides;
pub mod amines;
pub mod carboxylic_acids;
pub mod esters;
pub mod ethers;
pub mod fatty_acids;
pub mod halogenated;
pub mod heterocycles;
pub mod ketones;
pub mod nitriles_nitro;
pub mod organophosphorus;
pub mod organosulfur;

pub fn all() -> Vec<Molecule> {
    let mut v = vec![];
    v.extend(alcohols::all());
    v.extend(aldehydes::all());
    v.extend(ketones::all());
    v.extend(carboxylic_acids::all());
    v.extend(fatty_acids::all());
    v.extend(esters::all());
    v.extend(ethers::all());
    v.extend(amines::all());
    v.extend(amides::all());
    v.extend(nitriles_nitro::all());
    v.extend(halogenated::all());
    v.extend(organosulfur::all());
    v.extend(organophosphorus::all());
    v.extend(heterocycles::all());
    v
}
