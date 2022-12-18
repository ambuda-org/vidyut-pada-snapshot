//! Runs a full evaluation script over all program output.
//!
//! Usage: `make eval`
use clap::Parser;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use vidyut_prakriya::Ashtadhyayi;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    test_cases: PathBuf,

    #[arg(long)]
    hash: String,

    #[arg(long)]
    la: Option<String>,

    #[arg(long)]
    code: Option<String>,
}

fn calculate_sha256_file_hash(path: &Path) -> std::io::Result<String> {
    let mut hasher = Sha256::new();
    let mut file = File::open(path)?;
    std::io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // Check that the test file is as expected.
    let hash = calculate_sha256_file_hash(&args.test_cases)?;
    assert_eq!(hash, args.hash);

    let a = Ashtadhyayi::builder().log_steps(false).build();

    let mut rdr = csv::Reader::from_path(&args.test_cases)?;

    let mut num_matches = 0;
    let mut n = 0;

    let la_filter = args.la.map(|x| x.parse().unwrap());

    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let expected: Vec<_> = r[0].split('|').collect();
        let dhatu = &r[1];
        let code = String::from(&r[2]) + "." + &r[3];

        let prayoga = r[4].parse()?;
        let la = r[5].parse()?;
        let purusha = r[6].parse()?;
        let vacana = r[7].parse()?;

        // Filter by args
        if let Some(x) = la_filter {
            if la != x {
                continue;
            }
        }
        if let Some(x) = &args.code {
            if code != *x {
                continue;
            }
        }

        let prakriyas = a.derive_tinantas(dhatu, &code, la, prayoga, purusha, vacana);
        let mut actual: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
        actual.sort();

        n += 1;
        if expected == actual {
            num_matches += 1;
        } else {
            let la = &r[5];
            let purusha = &r[6];
            let vacana = &r[7];
            println!("[ FAIL ]  {code:<10} {dhatu:<10} {la:<10} {purusha:<10} {vacana:<10}");
            println!("          Expected: {:?}", expected);
            println!("          Actual  : {:?}", actual);
        }
    }

    let pct = 100_f32 * (num_matches as f32) / (n as f32);
    println!("{num_matches} / {n} tests pass ({pct:.2}%)");
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
