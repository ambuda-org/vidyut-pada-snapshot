/*!
Utility functions for working with the Dhatupatha.


*/

use crate::args::Dhatu;
use crate::term::Term;
use std::error::Error;
use std::path::Path;

/// Loads a list of dhatus from the given path.
pub fn load_dhatus(path: &Path) -> Result<Vec<Dhatu>, Box<dyn Error>> {
    let mut res = vec![];
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let code = &r[0];
        let upadesha = &r[1];

        if upadesha == "-" {
            continue;
        }
        if let Some((gana, number)) = code.split_once('.') {
            res.push(Dhatu::new(upadesha, gana.parse()?, number.parse()?));
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
