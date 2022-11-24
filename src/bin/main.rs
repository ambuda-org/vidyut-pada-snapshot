use std::error::Error;
use std::path::Path;
use vidyut_prakriya::ashtadhyayi as A;
use vidyut_prakriya::constants::{Prayoga, Purusha, Vacana};
use vidyut_prakriya::dhatupatha as D;

const LAKARA: &[&str] = &[
    "la~w",
    "li~w",
    "lu~w",
    "lf~w",
    "lo~w",
    "la~N",
    "ashir-lin",
    "li~N",
    "lu~N",
    "lf~N",
];

const TIN: &[&str] = &[
    "tip", "tas", "Ji", "sip", "Tas", "Ta", "mip", "vas", "mas", "ta", "AtAm", "Ja", "tAs", "ATAm",
    "Dvam", "iw", "vahi", "mahiN",
];

const TIN_SEMANTICS: &[(Purusha, Vacana)] = &[
    (Purusha::Prathama, Vacana::Eka),
    (Purusha::Prathama, Vacana::Dvi),
    (Purusha::Prathama, Vacana::Bahu),
    (Purusha::Madhyama, Vacana::Eka),
    (Purusha::Madhyama, Vacana::Dvi),
    (Purusha::Madhyama, Vacana::Bahu),
    (Purusha::Uttama, Vacana::Eka),
    (Purusha::Uttama, Vacana::Dvi),
    (Purusha::Uttama, Vacana::Bahu),
];

fn run() -> Result<(), Box<dyn Error>> {
    let dhatus = D::load_dhatus(Path::new("data/dhatupatha.tsv"));
    for dhatu in dhatus?.iter() {
        if let Some(la) = LAKARA.iter().next() {
            if let Some((purusha, vacana)) = TIN_SEMANTICS.iter().next() {
                let p = A::tinanta(
                    &dhatu.upadesha,
                    &dhatu.code(),
                    la,
                    Prayoga::Kartari,
                    *purusha,
                    *vacana,
                )?;
                println!("{}", p.text());
            }
        }
    }
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
