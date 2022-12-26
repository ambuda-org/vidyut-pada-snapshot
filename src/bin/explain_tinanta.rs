//! Debugging tool that displays all prakriyas available for the given pada and code.
//!
//! Usage:
//!
//! ```ignore
//! $ cargo run --bin explain -- --code 01.0001 --pada BavAmi
//! ```
use clap::Parser;
use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;
use vidyut_prakriya::args::{Lakara, Prayoga, Purusha, TinantaArgs, Vacana};
use vidyut_prakriya::dhatupatha as D;
use vidyut_prakriya::Ashtadhyayi;
use vidyut_prakriya::Prakriya;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    code: String,
    #[arg(long)]
    pada: String,
}

fn pretty_print_prakriya(p: &Prakriya) {
    println!("------------------------------");
    for step in p.history() {
        println!("{:<10} | {}", step.rule(), step.result());
    }
    println!("------------------------------");
    for choice in p.rule_choices() {
        println!("{choice:?}");
    }
    println!("------------------------------");
}

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

const PURUSHA_VACANA: &[(Purusha, Vacana)] = &[
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

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let dhatus = D::load_all(Path::new("data/dhatupatha.tsv"));

    let mut ordered_words = BTreeMap::new();
    let a = Ashtadhyayi::new();

    let (gana, number) = match args.code.split_once('.') {
        Some((x, y)) => (x.parse::<u8>()?, y.parse::<u16>()?),
        _ => return Ok(()),
    };

    for (dhatu, dhatu_number) in dhatus?.iter() {
        if !(dhatu.gana == gana && *dhatu_number == number) {
            continue;
        }
        for (i, lakara) in LAKARA.iter().enumerate() {
            let mut words = vec![];
            for (purusha, vacana) in PURUSHA_VACANA {
                let tinanta_args = TinantaArgs::builder()
                    .prayoga(Prayoga::Kartari)
                    .purusha(*purusha)
                    .vacana(*vacana)
                    .lakara(*lakara)
                    .build()?;

                let ps = a.derive_tinantas(dhatu, &tinanta_args);
                for p in ps {
                    words.push(p.text());
                    if p.text() == args.pada {
                        println!("{:?} {:?} {:?}", lakara, purusha, vacana);
                        pretty_print_prakriya(&p);
                    }
                }
            }
            ordered_words.insert(i, words);
        }
    }

    for (i, padas) in ordered_words.iter() {
        let la = LAKARA[*i];
        let data = padas.join(", ");
        println!("{la:?}: {data}");
    }
    Ok(())
}

fn main() {
    let args = Args::parse();

    match run(args) {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
