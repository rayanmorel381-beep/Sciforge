use super::super::Molecule;

pub mod amylose;
pub mod arabinose;
pub mod cellobiose;
pub mod cellulose;
pub mod chitin;
pub mod deoxyribose;
pub mod dextrin;
pub mod fructose;
pub mod galactose;
pub mod glucose;
pub mod glycogen;
pub mod inulin;
pub mod lactose;
pub mod maltose;
pub mod mannose;
pub mod ribose;
pub mod starch;
pub mod sucrose;
pub mod trehalose;
pub mod xylose;

pub fn all() -> Vec<Molecule> {
    vec![
        glucose::molecule(),
        fructose::molecule(),
        galactose::molecule(),
        mannose::molecule(),
        ribose::molecule(),
        deoxyribose::molecule(),
        xylose::molecule(),
        arabinose::molecule(),
        sucrose::molecule(),
        lactose::molecule(),
        maltose::molecule(),
        cellobiose::molecule(),
        trehalose::molecule(),
        starch::molecule(),
        amylose::molecule(),
        glycogen::molecule(),
        cellulose::molecule(),
        chitin::molecule(),
        inulin::molecule(),
        dextrin::molecule(),
    ]
}
