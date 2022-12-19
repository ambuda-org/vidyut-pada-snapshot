use crate::args::{Vacana, Vibhakti};
use crate::constants::Tag as T;
use crate::it_samjna;
use crate::prakriya::Prakriya;
use crate::term::Term;

#[allow(unused)]
fn find_sup(vibhakti: Vibhakti, vacana: Vacana) -> &'static str {
    match (vibhakti, vacana) {
        (Vibhakti::Prathama, Vacana::Eka) => "su~",
        (Vibhakti::Prathama, Vacana::Dvi) => "O",
        (Vibhakti::Prathama, Vacana::Bahu) => "jas",
        (Vibhakti::Dvitiya, Vacana::Eka) => "am",
        (Vibhakti::Dvitiya, Vacana::Dvi) => "Ow",
        (Vibhakti::Dvitiya, Vacana::Bahu) => "Sas",
        (Vibhakti::Trtiya, Vacana::Eka) => "wA",
        (Vibhakti::Trtiya, Vacana::Dvi) => "ByAm",
        (Vibhakti::Trtiya, Vacana::Bahu) => "Bis",
        (Vibhakti::Caturthi, Vacana::Eka) => "Ne",
        (Vibhakti::Caturthi, Vacana::Dvi) => "ByAm",
        (Vibhakti::Caturthi, Vacana::Bahu) => "Byas",
        (Vibhakti::Panchami, Vacana::Eka) => "Nasi~",
        (Vibhakti::Panchami, Vacana::Dvi) => "ByAm",
        (Vibhakti::Panchami, Vacana::Bahu) => "Byas",
        (Vibhakti::Sasthi, Vacana::Eka) => "Nas",
        (Vibhakti::Sasthi, Vacana::Dvi) => "os",
        (Vibhakti::Sasthi, Vacana::Bahu) => "Am",
        (Vibhakti::Saptami, Vacana::Eka) => "Ni",
        (Vibhakti::Saptami, Vacana::Dvi) => "os",
        (Vibhakti::Saptami, Vacana::Bahu) => "sup",
    }
}

#[allow(unused)]
pub fn run(p: &mut Prakriya, vibhakti: Vibhakti, vacana: Vacana) -> Option<()> {
    let sup = find_sup(vibhakti, vacana);
    let mut sup = Term::make_upadesha(sup);

    sup.add_tag(T::Pratyaya);
    sup.add_tag(T::Sup);
    p.terms_mut().push(sup);
    p.step("4.1.2");

    let i = p.terms().len() - 1;
    it_samjna::run(p, i).ok()?;

    Some(())
}
