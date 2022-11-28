use std::error::Error;
use std::path::Path;
use vidyut_prakriya::ashtadhyayi as A;
use vidyut_prakriya::constants::{La, Prayoga, Purusha, Vacana};
use vidyut_prakriya::dhatupatha as D;

const LAKARA: &[La] = &[
    La::Lat,
    La::Lit,
    La::Lut,
    La::Lrt,
    La::Let,
    La::Lot,
    La::Lan,
    La::AshirLin,
    La::VidhiLin,
    La::Lun,
    La::Lrn,
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
        for la in LAKARA {
            for (purusha, vacana) in TIN_SEMANTICS {
                let p = A::tinanta(
                    &dhatu.upadesha,
                    &dhatu.code(),
                    *la,
                    Prayoga::Kartari,
                    *purusha,
                    *vacana,
                )?;
                println!("{la:?}: {}", p.text());
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
