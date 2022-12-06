use clap::Parser;
use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;
use vidyut_gen::ashtadhyayi as A;
use vidyut_gen::constants::{La, Prayoga, Purusha, Vacana};
use vidyut_gen::dhatupatha as D;
use vidyut_gen::prakriya::Prakriya;

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
        println!("{:<10} | {}", step.rule, step.state);
    }
    println!("------------------------------");
    for choice in p.rule_choices() {
        println!("{choice:?}");
    }
    println!("------------------------------");
}

const LAKARA: &[La] = &[
    La::Lat,
    La::Lit,
    La::Lut,
    La::Lrt,
    La::Lot,
    La::Lan,
    La::AshirLin,
    La::VidhiLin,
    La::Lun,
    La::Lrn,
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
    let dhatus = D::load_dhatus(Path::new("data/dhatupatha.tsv"));

    let mut ordered_words = BTreeMap::new();
    for dhatu in dhatus?.iter() {
        if dhatu.code() != args.code {
            continue;
        }
        for (i, la) in LAKARA.iter().enumerate() {
            let mut words = vec![];
            for (purusha, vacana) in PURUSHA_VACANA {
                let ps = A::derive_tinantas(
                    &dhatu.upadesha,
                    &dhatu.code(),
                    *la,
                    Prayoga::Kartari,
                    *purusha,
                    *vacana,
                    true,
                );
                for p in ps {
                    words.push(p.text());
                    if p.text() == args.pada {
                        println!("{:?} {:?} {:?}", la, purusha, vacana);
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
