use clap::Parser;
use std::error::Error;
use std::path::Path;
use vidyut_prakriya::ashtadhyayi as A;
use vidyut_prakriya::constants::{La, Prayoga, Purusha, Vacana};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    limit: Option<i32>,
}

fn parse_la(s: &str) -> La {
    match s {
        "law" => La::Lat,
        "liw" => La::Lit,
        "luw" => La::Lut,
        "lfw" => La::Lrt,
        "lew" => La::Let,
        "low" => La::Lot,
        "laN" => La::Lan,
        "liN" => La::VidhiLin,
        "ASIrliN" => La::AshirLin,
        "luN" => La::Lun,
        "lfN" => La::Lrn,
        _ => panic!("Unknown {s}"),
    }
}

fn parse_purusha(s: &str) -> Purusha {
    match s {
        "prathama" => Purusha::Prathama,
        "madhyama" => Purusha::Madhyama,
        "uttama" => Purusha::Uttama,
        _ => panic!("Unknown {s}"),
    }
}

fn parse_vacana(s: &str) -> Vacana {
    match s {
        "eka" => Vacana::Eka,
        "dvi" => Vacana::Dvi,
        "bahu" => Vacana::Bahu,
        _ => panic!("Unknown {s}"),
    }
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("data/eval.csv")?;

    let mut num_matches = 0;
    let mut n = 0;
    let limit = args.limit.unwrap_or(std::i32::MAX);

    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let pada = &r[0];
        let dhatu = &r[1];
        let code = String::from(&r[2]) + "." + &r[3];

        let la = parse_la(&r[4]);
        let purusha = parse_purusha(&r[5]);
        let vacana = parse_vacana(&r[6]);

        let p = A::tinanta(dhatu, &code, la, Prayoga::Kartari, purusha, vacana)?;

        n += 1;
        let actual = p.text();
        if actual == pada {
            println!("[  OK  ]: {pada}");
            num_matches += 1;
        } else {
            println!("[ FAIL ]: {pada} ({actual})");
        }

        if n >= limit {
            break;
        }
    }

    let p = A::tinanta(
        "BU",
        "1.0001",
        La::Lot,
        Prayoga::Kartari,
        Purusha::Uttama,
        Vacana::Dvi,
    )?;
    println!("------------------------------");
    for step in p.history() {
        println!("{:<8} | {}", step.rule, step.state);
    }
    println!("------------------------------");

    let pct = 100_f32 * (num_matches as f32) / (n as f32);
    println!("Results: {num_matches} / {n} ({pct:.2}%)");
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
