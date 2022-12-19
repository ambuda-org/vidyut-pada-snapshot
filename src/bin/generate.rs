//! Generates all tinantas currently supported by the crate.
//!
//! Usage: `make generate`
use serde::Serialize;
use std::io;
use std::path::Path;
use vidyut_prakriya::args::{Dhatu, Lakara, Prayoga, Purusha, Vacana};
use vidyut_prakriya::dhatupatha as D;
use vidyut_prakriya::Ashtadhyayi;

const LAKARA: &[Lakara] = &[
    Lakara::Lat,
    Lakara::Lit,
    Lakara::Lut,
    Lakara::Lrt,
    Lakara::Let,
    Lakara::Lot,
    Lakara::Lan,
    Lakara::AshirLin,
    Lakara::VidhiLin,
    Lakara::Lun,
    Lakara::Lrn,
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

#[derive(Debug, Serialize)]
struct Row<'a> {
    pada: String,
    dhatu: &'a str,
    gana: i32,
    number: i32,
    prayoga: &'static str,
    lakara: &'static str,
    purusha: &'static str,
    vacana: &'static str,
}

fn run(dhatus: Vec<Dhatu>) -> Result<(), io::Error> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let a = Ashtadhyayi::builder().log_steps(false).build();

    for dhatu in dhatus {
        for la in LAKARA {
            for (purusha, vacana) in TIN_SEMANTICS {
                let prayoga = Prayoga::Kartari;
                let prakriyas = a.derive_tinantas(&dhatu, *la, prayoga, *purusha, *vacana);

                let dhatu_text = &dhatu.upadesha;
                for p in prakriyas {
                    let row = Row {
                        pada: p.text().to_string(),
                        dhatu: dhatu_text,
                        gana: dhatu.gana as i32,
                        number: dhatu.number as i32,
                        lakara: la.as_str(),
                        purusha: purusha.as_str(),
                        vacana: vacana.as_str(),
                        prayoga: prayoga.as_str(),
                    };

                    wtr.serialize(row)?;
                }
            }
        }
    }

    wtr.flush()?;
    Ok(())
}

fn main() {
    let dhatus = match D::load_dhatus(Path::new("data/dhatupatha.tsv")) {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    match run(dhatus) {
        Ok(()) => (),
        // FIXME: match doesn't seem to work on osx?
        Err(err) if err.kind() == std::io::ErrorKind::BrokenPipe => (),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
