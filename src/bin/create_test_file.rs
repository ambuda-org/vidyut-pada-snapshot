//! Usage: `make generate_test_file`
use serde::Serialize;
use std::io;
use std::path::Path;
use vidyut_prakriya::args::{Dhatu, Lakara, Prayoga, Purusha, Vacana};
use vidyut_prakriya::dhatupatha as D;
use vidyut_prakriya::Ashtadhyayi;

// TODO: reuse with other binaries?
const LAKARA: &[Lakara] = &[
    Lakara::Lat,
    Lakara::Lit,
    Lakara::Lut,
    Lakara::Lrt,
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
    padas: String,
    dhatu: &'a str,
    gana: u8,
    number: u16,
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
                let mut padas: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
                padas.sort();
                let padas = padas.join("|");

                let row = Row {
                    padas,
                    dhatu: dhatu_text,
                    gana: dhatu.gana,
                    number: dhatu.number,
                    lakara: la.as_str(),
                    purusha: purusha.as_str(),
                    vacana: vacana.as_str(),
                    prayoga: prayoga.as_str(),
                };

                wtr.serialize(row)?;
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
