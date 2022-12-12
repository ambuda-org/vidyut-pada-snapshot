use serde::Serialize;
use std::io;
use std::path::Path;
use vidyut_gen::arguments::{La, Prayoga, Purusha, Vacana};
use vidyut_gen::ashtadhyayi as A;
use vidyut_gen::dhatupatha as D;

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

fn run(dhatus: Vec<D::Dhatu>) -> Result<(), io::Error> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    for dhatu in dhatus {
        for la in LAKARA {
            for (purusha, vacana) in TIN_SEMANTICS {
                let prayoga = Prayoga::Kartari;
                let prakriyas = A::derive_tinantas(
                    &dhatu.upadesha,
                    &dhatu.code(),
                    *la,
                    prayoga,
                    *purusha,
                    *vacana,
                    false,
                );

                let dhatu_text = &dhatu.upadesha;
                for p in prakriyas {
                    let row = Row {
                        pada: p.text().to_string(),
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
