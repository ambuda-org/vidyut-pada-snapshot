use crate::constants::La;
use crate::constants::Tag as T;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::{Prakriya, Rule};
use crate::term::Term;
use std::error::Error;

fn add_sanadi(
    rule: Rule,
    p: &mut Prakriya,
    index: usize,
    upadesha: &str,
) -> Result<(), Box<dyn Error>> {
    let mut pratyaya = Term::make_upadesha(upadesha);
    pratyaya.add_tags(&[T::Pratyaya]);
    p.insert_after(index, pratyaya);
    p.step(rule);

    let pratyaya_index = index + 1;
    p.term_rule(
        "3.1.32",
        pratyaya_index,
        |_| true,
        |t| op::samjna(t, T::Dhatu),
    );
    it_samjna::run(p, pratyaya_index)?;

    Ok(())
}

// TODO: 3.1.8 - 3.1.24
// TODO: 3.1.26 - 3.1.27
pub fn run(p: &mut Prakriya, la: La) -> Result<(), Box<dyn Error>> {
    let i = match p.find_first(T::Dhatu) {
        Some(i) => i,
        None => return Ok(()),
    };

    // These dhatus use san-pratyaya with a long abhyAsa.
    const MAN_BADHA: &[&str] = &["mAna~\\", "baDa~\\", "dAna~^", "SAna~^"];
    // These dhatus use their pratyaya optionally if followed by ArdhadhAtuka.
    const AYADAYA: &[&str] = &[
        "gupU~", "DUpa~", "vicCa~", "paRa~\\", "pana~\\", "fti", "kamu~\\",
    ];

    // `gana` is required so that we can exclude "03.0021 kita~".
    if p.has(i, |t| {
        t.has_u_in(&["gupa~\\", "tija~\\", "kita~"]) && t.gana == Some(1)
    }) {
        add_sanadi("3.1.5", p, i, "san")?;
        p.set(i + 1, |t| t.add_tag(T::FlagNoArdhadhatuka));
    } else if p.has(i, |t| t.has_u_in(MAN_BADHA)) {
        add_sanadi("3.1.6", p, i, "san")?;
        p.set(i + 1, |t| t.add_tag(T::FlagNoArdhadhatuka));
    } else if p.has(i, |t| t.gana == Some(10)) {
        add_sanadi("3.1.25", p, i, "Ric")?;
    } else if p.has(i, |t| t.has_u_in(AYADAYA)) {
        let mut add_pratyaya = true;

        if la.is_ardhadhatuka() {
            if p.is_allowed("3.1.31") {
                add_pratyaya = false;
                p.step("3.1.31");
            } else {
                p.decline("3.1.31");
            }
        }

        if add_pratyaya {
            if p.has(i, |t| {
                t.has_u_in(&["gupU~", "DUpa~", "vicCa~", "paRa~\\", "pana~\\"])
            }) {
                add_sanadi("3.1.28", p, i, "Aya")?;
            } else if p.has(i, |t| t.has_u("fti")) {
                add_sanadi("3.1.29", p, i, "IyaN")?;
            } else if p.has(i, |t| t.has_u("kamu~\\")) {
                add_sanadi("3.1.30", p, i, "RiN")?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dhatu_karya;

    fn check(dhatu: &str, code: &str) -> (Term, Term) {
        let mut p = Prakriya::new();
        dhatu_karya::run(&mut p, dhatu, code).unwrap();

        run(&mut p, La::Lat).unwrap();
        let dhatu = p.get(0).unwrap();
        let pratyaya = p.get(1).unwrap();
        (dhatu.clone(), pratyaya.clone())
    }

    #[test]
    fn test_gup() {
        let (_, p) = check("gupa~\\", "01.1125");
        assert_eq!(p.text, "sa");
        assert!(p.all(&[T::Pratyaya, T::FlagNoArdhadhatuka]));
    }

    #[test]
    fn test_man() {
        let (_, p) = check("mAna~\\", "01.1127");
        assert_eq!(p.text, "sa");
        assert!(p.all(&[T::Pratyaya, T::FlagNoArdhadhatuka]));
    }

    #[test]
    fn test_curadi() {
        let (_, p) = check("cura~", "10.0001");
        assert_eq!(p.text, "i");
        assert!(p.has_tag(T::Pratyaya));
    }

    #[test]
    fn test_ayadaya() {
        let (_, p) = check("gupU~", "01.0461");
        assert_eq!(p.text, "Aya");
        assert!(p.has_tag(T::Pratyaya));

        let (_, p) = check("fti", "01.1166");
        assert_eq!(p.text, "Iya");
        assert!(p.all(&[T::Pratyaya, T::Nit]));

        let (_, p) = check("kamu~\\", "01.0511");
        assert_eq!(p.text, "i");
        assert!(p.all(&[T::Pratyaya, T::Rit, T::Nit]));
    }
}
