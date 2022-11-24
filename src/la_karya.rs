use crate::constants::Tag as T;
use crate::it_samjna;
use crate::prakriya::{Prakriya, Rule};
use crate::term::Term;
use std::error::Error;

fn add_la(rule: Rule, p: &mut Prakriya, i: usize, la: &str) -> Result<(), Box<dyn Error>> {
    let mut la = Term::make_upadesha(la);
    la.add_tag(T::Pratyaya);

    p.insert_after(i, la);
    p.step(rule);
    it_samjna::run(p, i + 1)?;

    Ok(())
}

pub fn run(p: &mut Prakriya, la: &str) -> Result<(), Box<dyn Error>> {
    let i = match p.find_last(T::Dhatu) {
        Some(i) => i,
        None => return Ok(()),
    };

    match la {
        "la~w" => add_la("3.3.123", p, i, la)?,
        "li~w" => add_la("3.2.114", p, i, la)?,
        "lu~w" => add_la("3.3.15", p, i, la)?,
        "lf~w" => add_la("3.3.13", p, i, la)?,
        "le~w" => add_la("3.4.7", p, i, la)?,
        "lo~w" => add_la("3.3.162", p, i, la)?,
        "la~N" => add_la("3.2.111", p, i, la)?,
        "li~N" => {
            if p.has_tag(T::Ashih) {
                add_la("3.3.173", p, i, la)?;
            } else {
                add_la("3.3.161", p, i, la)?;
            }
        }
        "lu~N" => add_la("3.2.110", p, i, la)?,
        "lf~N" => add_la("3.3.139", p, i, la)?,
        _ => panic!("Unknown lakara"),
    };

    Ok(())
}
