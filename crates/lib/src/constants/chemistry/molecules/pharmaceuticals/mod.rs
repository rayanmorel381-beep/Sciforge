use super::Molecule;

pub mod albendazole;
pub mod amitriptyline;
pub mod amlodipine;
pub mod amoxicillin;
pub mod ampicillin;
pub mod artemisinin;
pub mod aspirin;
pub mod atenolol;
pub mod atorvastatin;
pub mod azithromycin;
pub mod caffeine;
pub mod captopril;
pub mod chloroquine;
pub mod ciprofloxacin;
pub mod codeine;
pub mod dexamethasone;
pub mod diazepam;
pub mod diphenhydramine;
pub mod doxycycline;
pub mod enalapril;
pub mod ethambutol;
pub mod fluoxetine;
pub mod furosemide;
pub mod glibenclamide;
pub mod haloperidol;
pub mod hydrochlorothiazide;
pub mod hydrocortisone;
pub mod hydroxychloroquine;
pub mod ibuprofen;
pub mod isoniazid;
pub mod ivermectin;
pub mod lisinopril;
pub mod loratadine;
pub mod lorazepam;
pub mod mebendazole;
pub mod metformin;
pub mod metronidazole;
pub mod morphine;
pub mod omeprazole;
pub mod paracetamol;
pub mod penicillin_g;
pub mod prednisolone;
pub mod propranolol;
pub mod ranitidine;
pub mod rifampicin;
pub mod salbutamol;
pub mod sertraline;
pub mod simvastatin;
pub mod sulfamethoxazole;
pub mod tramadol;
pub mod trimethoprim;
pub mod warfarin;

pub fn all() -> Vec<Molecule> {
    vec![
        aspirin::molecule(),
        paracetamol::molecule(),
        ibuprofen::molecule(),
        caffeine::molecule(),
        amoxicillin::molecule(),
        ampicillin::molecule(),
        penicillin_g::molecule(),
        ciprofloxacin::molecule(),
        doxycycline::molecule(),
        azithromycin::molecule(),
        metronidazole::molecule(),
        sulfamethoxazole::molecule(),
        trimethoprim::molecule(),
        isoniazid::molecule(),
        rifampicin::molecule(),
        ethambutol::molecule(),
        chloroquine::molecule(),
        hydroxychloroquine::molecule(),
        artemisinin::molecule(),
        ivermectin::molecule(),
        albendazole::molecule(),
        mebendazole::molecule(),
        metformin::molecule(),
        glibenclamide::molecule(),
        atorvastatin::molecule(),
        simvastatin::molecule(),
        omeprazole::molecule(),
        ranitidine::molecule(),
        loratadine::molecule(),
        diphenhydramine::molecule(),
        salbutamol::molecule(),
        prednisolone::molecule(),
        dexamethasone::molecule(),
        hydrocortisone::molecule(),
        morphine::molecule(),
        codeine::molecule(),
        tramadol::molecule(),
        diazepam::molecule(),
        lorazepam::molecule(),
        fluoxetine::molecule(),
        sertraline::molecule(),
        amitriptyline::molecule(),
        haloperidol::molecule(),
        warfarin::molecule(),
        enalapril::molecule(),
        lisinopril::molecule(),
        amlodipine::molecule(),
        atenolol::molecule(),
        propranolol::molecule(),
        furosemide::molecule(),
        captopril::molecule(),
        hydrochlorothiazide::molecule(),
    ]
}
