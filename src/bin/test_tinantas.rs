//! Runs a full evaluation script over all program output.
//!
//! Usage: `make eval`
use clap::Parser;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use vidyut_prakriya::args::TinantaArgs;
use vidyut_prakriya::dhatupatha;
use vidyut_prakriya::Ashtadhyayi;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    test_cases: PathBuf,

    #[arg(long)]
    hash: String,
}

fn calculate_sha256_file_hash(path: &Path) -> std::io::Result<String> {
    let mut hasher = Sha256::new();
    let mut file = File::open(path)?;
    std::io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

fn check_file_hash(args: &Args) {
    // Check that the test file is as expected.
    let hash = match calculate_sha256_file_hash(&args.test_cases) {
        Ok(x) => x,
        Err(err) => {
            println!(
                "We could not create a hash for {}",
                args.test_cases.display()
            );
            println!("Error was: {}", err);
            std::process::exit(1);
        }
    };
    if hash != args.hash {
        println!();
        println!("The test file has test cases that differ from the ones we were expecting.");
        println!("We know this because the test file has an unexpected hash value:");
        println!();
        println!("    Path to test file: {}", args.test_cases.display());
        println!("    Expected hash    : {}", args.hash);
        println!("    Actual hash      : {}", hash);
        println!();
        println!(
            "If you are intentionally trying to change the test file -- for example, because you"
        );
        println!("are changing the implementation of some rule -- then please open `Makefile` and");
        println!(
            "replace the hash value in the `test_all` command with the `Actual hash` value above."
        );
        println!();
        println!(
            "If you have not changed any core code, please file a GitHub issue so that we can help"
        );
        println!("you debug the issue (https://github.com/ambuda-org/vidyut/issues/).");
        println!();
        std::process::exit(1);
    }

    assert_eq!(hash, args.hash);
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    check_file_hash(&args);

    let a = Ashtadhyayi::builder().log_steps(false).build();

    let mut rdr = csv::Reader::from_path(&args.test_cases)?;

    let mut num_matches = 0;
    let mut n = 0;

    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let expected: Vec<_> = r[0].split('|').collect();
        let dhatu = dhatupatha::resolve(&r[1], &r[2], &r[3])?;

        let prayoga = r[4].parse()?;
        let lakara = r[5].parse()?;
        let purusha = r[6].parse()?;
        let vacana = r[7].parse()?;

        let tinanta_args = TinantaArgs::builder()
            .prayoga(prayoga)
            .purusha(purusha)
            .vacana(vacana)
            .lakara(lakara)
            .build()?;

        let prakriyas = a.derive_tinantas(&dhatu, &tinanta_args);
        let mut actual: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
        actual.sort();

        n += 1;
        if expected == actual {
            num_matches += 1;
        } else {
            let lakara = &r[5];
            let purusha = &r[6];
            let vacana = &r[7];
            let code = dhatu.code(&r[3]);
            let upadesha = dhatu.upadesha;
            println!("[ FAIL ]  {code:<10} {upadesha:<10} {lakara:<10} {purusha:<10} {vacana:<10}");
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
