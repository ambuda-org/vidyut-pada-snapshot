use crate::term::Term;
use std::error::Error;
use std::path::Path;

pub struct Dhatu {
    pub upadesha: String,
    pub gana: i32,
    pub number: i32,
}

impl Dhatu {
    pub fn code(&self) -> String {
        format!("{:0>2}.{:0>4}", self.gana, self.number)
    }
}

pub fn load_dhatus(path: &Path) -> Result<Vec<Dhatu>, Box<dyn Error>> {
    let mut res = vec![];
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let code = r[0].to_string();
        let upadesha = r[1].to_string();

        if upadesha == "-" {
            continue;
        }
        if let Some((gana, number)) = code.split_once('.') {
            res.push(Dhatu {
                upadesha,
                gana: gana.parse()?,
                number: number.parse()?,
            });
        }
    }
    Ok(res)
}

pub fn is_kutadi(t: &Term) -> bool {
    // Check number explicitly because some roots are duplicated within tudAdi
    // but outside this gana (e.g. juq).
    match t.number {
        Some(n) => t.has_gana(6) && (93..=137).contains(&n),
        _ => false,
    }
}
