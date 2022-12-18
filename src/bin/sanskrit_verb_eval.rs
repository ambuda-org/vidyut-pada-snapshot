//! Runs a full evaluation script over all program output.
//!
//! Usage: `make eval`
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::Path;
use vidyut_gen::arguments::{Prayoga, Purusha, Vacana};
use vidyut_gen::ashtadhyayi as A;
use vidyut_gen::dhatupatha as D;

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

fn run() -> Result<(), Box<dyn Error>> {
    let dhatus = D::load_dhatus(Path::new("data/dhatupatha.tsv"))?;
    let dhatu_map: HashMap<_, _> = dhatus.into_iter().map(|d| (d.code(), d.upadesha)).collect();

    let sv_code_to_vidyut_code: HashMap<_, _> = vec![
        ("10.0005", "10.0121"),
        ("10.0019", "10.0409"),
        ("06.0033", "06.0032"),
        ("10.0146", "10.0310"),
        // Bad names
        ("01.0150", "01.0151"),
        ("02.0111", "02.0013"),
        ("02.0127", "02.0037"),
        ("02.0183", "02.0009"),
        ("02.0321", "02.0028"),
        ("02.0342", "02.0008"),
        ("02.0380", "02.0039"),
        ("02.0386", "02.0061"),
        ("03.0152", "03.0015"),
        ("03.0318", "03.0006"),
        ("04.0168", "04.0067"),
        ("04.0184", "04.0108"),
        ("04.0187", "04.0143"),
        ("04.0190", "04.0151"),
        ("04.0191", "04.0152"),
        ("04.0195", "04.0110"),
        ("04.0197", "04.0145"),
        ("04.0217", "04.0049"),
        ("04.0218", "04.0098"),
        ("04.0229", "04.0105"),
        ("04.0249", "04.0001"),
        ("04.0269", "04.0011"),
        ("04.0273", "04.0111"),
        ("04.0276", "04.0115"),
        ("04.0294", "04.0126"),
        ("04.0302", "04.0147"),
        ("04.0306", "04.0013"),
        ("04.0310", "04.0146"),
        ("04.0312", "04.0055"),
        ("04.0334", "04.0046"),
        ("04.0341", "04.0023"),
        ("04.0350", "04.0054"),
        ("04.0387", "04.0060"),
        ("05.0124", "05.0005"),
        ("05.0133", "05.0009"),
        ("05.0351", "05.0028"),
        ("05.0388", "05.0025"),
        ("05.0476", "05.0011"),
        ("05.0540", "05.0031"),
        ("06.0189", "06.0098"),
        ("06.0221", "06.0102"),
        ("06.0222", "06.0093"),
        ("06.0237", "06.0019"),
        ("06.0247", "06.0100"),
        ("06.0272", "06.0166"),
        ("06.0283", "06.0094"),
        ("06.0284", "06.0109"),
        ("06.0293", "06.0141"),
        ("06.0296", "06.0136"),
        ("06.0335", "06.0152"),
        ("06.0355", "06.0028"),
        ("06.0356", "06.0036"),
        ("06.0357", "06.0048"),
        ("06.0371", "06.0008"),
        ("06.0835", "06.0147"),
        ("07.0232", "07.0013"),
        ("07.0290", "07.0016"),
        ("07.0316", "07.0021"),
        ("07.0338", "07.0007"),
        ("07.0339", "07.0025"),
        ("07.0344", "07.0024"),
        ("07.0348", "07.0004"),
        ("07.0349", "07.0014"),
        ("07.0352", "07.0008"),
        ("07.0366", "07.0019"),
        ("08.0233", "08.0009"),
        ("08.0377", "08.0001"),
        ("09.0231", "09.0033"),
        ("09.0235", "09.0011"),
        ("09.0258", "09.0043"),
        ("09.0270", "09.0060"),
        ("09.0280", "09.0065"),
        ("09.0299", "09.0019"),
        ("09.0343", "09.0036"),
        ("09.0346", "09.0027"),
        ("09.0361", "09.0004"),
        ("09.0372", "09.0020"),
        ("09.0373", "09.0002"),
        ("09.0374", "09.0046"),
        ("09.0375", "09.0049"),
        ("10.0008", "10.0342"),
        ("10.0009", "10.0183"),
        ("10.0021", "10.0316"),
        ("10.0093", "10.0404"),
        ("10.0146", "10.0310"),
        ("10.0837", "10.0345"),
        ("10.1045", "10.0318"),
        ("10.1103", "10.0296"),
        ("10.1114", "10.0319"),
        // Swapped pairs
        ("01.0073", "01.0878"),
        ("01.0878", "01.0073"),
        ("01.0074", "01.0879"),
        ("01.0879", "01.0074"),
        ("01.0075", "01.0880"),
        ("01.0880", "01.0075"),
        ("01.0098", "01.0892"),
        ("01.0892", "01.0098"),
        ("01.0187", "01.1152"),
        ("01.1152", "01.0187"),
        ("01.0215", "01.0998"),
        ("01.0998", "01.0215"),
        ("01.0392", "01.0852"),
        ("01.0852", "01.0392"),
        ("01.0957", "01.0205"),
        ("01.0205", "01.0957"),
        ("06.0002", "06.0162"),
        ("06.0162", "06.0002"),
        ("01.1115", "01.1047"),
        ("01.1047", "01.1115"),
        ("01.0864", "01.1014"),
        ("01.1014", "01.0864"),
        ("01.0546", "01.1031"),
        ("01.1031", "01.0546"),
        // Shift down
        ("02.0076", "02.0077"),
        // Swapped triples
        ("01.1106", "01.0740"),
        ("01.0739", "01.1106"),
        ("01.0740", "01.0739"),
        ("01.0330", "01.0359"),
        ("01.0331", "01.0330"),
        ("01.0359", "01.0331"),
    ]
    .iter()
    .map(|(x, y)| (x.to_string(), y.to_string()))
    .collect();

    let mut rdr = csv::Reader::from_path("sanskrit-verb.csv")?;

    let mut num_added = 0;
    let mut num_removed = 0;
    for maybe_row in rdr.records() {
        let r = maybe_row?;

        let sv_code = &r[1].to_string();
        let v_code = sv_code_to_vidyut_code.get(sv_code).unwrap_or(sv_code);
        let dhatu = match dhatu_map.get(v_code) {
            Some(s) => s,
            None => {
                println!("Bad code: {v_code}");
                continue;
            }
        };
        if dhatu.is_empty() {
            println!("Bad code: {v_code}");
            continue;
        }

        let la = r[2].parse()?;
        let sv_padas: HashSet<_> = r[3].split('|').map(|t| t.to_string()).collect();

        let mut v_padas = HashSet::new();
        for (purusha, vacana) in PURUSHA_VACANA {
            let prakriyas = A::derive_tinantas(
                dhatu,
                v_code,
                la,
                Prayoga::Kartari,
                *purusha,
                *vacana,
                false,
            );
            v_padas.extend(
                prakriyas
                    .iter()
                    .map(|p| p.text().to_string())
                    // Filter out final 'd' words to avoid false positives.
                    .filter(|s| !s.ends_with('d')),
            );
        }

        if sv_padas != v_padas {
            let added: HashSet<_> = v_padas.difference(&sv_padas).collect();
            let removed: HashSet<_> = sv_padas.difference(&v_padas).collect();
            num_added += added.len();
            num_removed += removed.len();

            let la = &r[2];
            if num_added > 0 || num_removed > 0 {
                println!("[ DIFF ]  sv:{sv_code} / v:{v_code}    {dhatu:<10} {la:<10}");
                println!("          Added  : {:?}", added);
                println!("          Removed: {:?}", removed);
            }
        }
    }
    println!("{num_added} padas added");
    println!("{num_removed} padas removed");

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
