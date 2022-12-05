use crate::constants::Tag as T;
use crate::dhatu_gana as gana;
use crate::filters as f;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::term::Term;

use std::error::Error;

fn init(p: &mut Prakriya, dhatu: &str, code: &str) -> Result<(), Box<dyn Error>> {
    let (gana, number) = match code.split_once('.') {
        Some((x, y)) => (x, y),
        None => return Ok(()),
    };

    // The root enters the prakriyA
    p.push(Term::make_dhatu(dhatu, gana.parse()?, number.parse()?));
    p.step("start");

    Ok(())
}

fn add_samjnas(p: &mut Prakriya, i: usize) -> Result<(), Box<dyn Error>> {
    p.op_term("1.3.1", i, op::add_tag(T::Dhatu));
    it_samjna::run(p, i)?;

    if p.has(i, |t| {
        t.has_text_in(&["dA", "de", "do", "DA", "De"]) && !t.has_u("dA\\p")
    }) {
        p.op_term("1.1.20", i, op::add_tag(T::Ghu));
    };

    Ok(())
}

fn gana_sutras(p: &mut Prakriya, i: usize) {
    let d = p.get(i).unwrap();
    if let (Some(10), Some(num)) = (d.gana, d.number) {
        if p.has(i, f::u_in(gana::CUR_MIT)) {
            p.op("cur-mit", op::t(i, op::add_tag(T::mit)));
        }

        // Need to check range explicitly because some of these roots appear
        // multiple times in the gana, e.g. lakza~
        p.rule(
            "kusmadi",
            |p| {
                p.has(i, |t| {
                    t.has_u_in(gana::KUSMADI) && (192..=236).contains(&num)
                })
            },
            |p| p.add_tag(T::Atmanepada),
        );
        p.rule(
            "garvadi",
            |p| {
                p.has(i, |t| {
                    t.has_u_in(gana::GARVADI) && (440..=449).contains(&num)
                })
            },
            |p| p.add_tag(T::Atmanepada),
        );
    }
}

fn satva_and_natva(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get_mut(i)?;
    if dhatu.has_adi('z') {
        if dhatu.has_text_in(&["zWiv", "zvazk"]) {
            // Varttika -- no change for zWiv or zvask
            p.step("6.1.64.v1");
        } else if dhatu.has_prefix_in(&["zw", "zW", "zR"]) {
            // Varttika -- also change next sound
            match &dhatu.text[..2] {
                "zw" => dhatu.text.replace_range(..2, "st"),
                "zW" => dhatu.text.replace_range(..2, "sT"),
                "zR" => dhatu.text.replace_range(..2, "sn"),
                _ => (),
            };
            dhatu.add_tag(T::FlagAdeshadi);
            p.step("6.1.64.v2");
        } else {
            dhatu.add_tag(T::FlagAdeshadi);
            p.set(i, op::adi("s"));
            p.step("6.1.64");
        }
    } else if dhatu.has_adi('R') {
        dhatu.add_tag(T::FlagAdeshadi);
        p.set(i, op::adi("n"));
        p.step("6.1.65");
    }

    Some(())
}

// nu~m-Agama
//
// Although this rule is declared in the "aNgasya" section of the Ashtadhyayi, applying this rule
// there will cause problems, e.g. when applying 3.1.80 (dhinvikRNvyor a ca). To see why, try
// moving this rule and running the tests.
//
// TODO: why exception for cakz?
fn maybe_add_num_agama(p: &mut Prakriya, i: usize) {
    if p.has(i, |t| t.has_tag(T::idit) && !t.has_u("ca\\kzi~\\N")) {
        p.op_term("7.1.58", i, op::mit("n"));
    }
}

fn maybe_add_upasarga(p: &mut Prakriya, i: usize) {
    // These two roots are always used with the upasarga `adhi-`:
    p.rule(
        "1.4.80",
        |p| p.has(i, |t| t.has_u_in(&["i\\N", "i\\k"])),
        |p| {
            let mut upa = Term::make_upadesha("aDi");
            upa.add_tag(T::Upasarga);
            p.insert_before(0, upa);
        },
    );
}

pub fn run(p: &mut Prakriya, dhatu: &str, code: &str) -> Result<(), Box<dyn Error>> {
    let i = 0;

    init(p, dhatu, code)?;
    add_samjnas(p, i)?;
    gana_sutras(p, i);

    satva_and_natva(p, i);
    maybe_add_num_agama(p, i);
    maybe_add_upasarga(p, i);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(text: &str, code: &str) -> Term {
        let mut p = Prakriya::new();
        run(&mut p, text, code).unwrap();
        p.get(0).unwrap().clone()
    }

    #[test]
    fn test_basic() {
        let t = check("ga\\mx~", "01.1137");
        assert_eq!(t.text, "gam");
        assert!(t.has_tag(T::Dhatu));
    }

    #[test]
    fn test_ghu() {
        let t = check("qudA\\Y", "03.0010");
        assert_eq!(t.text, "dA");
        assert!(t.all(&[T::Dhatu, T::Ghu]));
    }

    #[test]
    fn test_satva() {
        let t = check("zaha~\\", "01.0988");
        assert_eq!(t.text, "sah");
        assert!(t.all(&[T::Dhatu, T::FlagAdeshadi]));

        let t = check("zWA\\", "01.1077");
        assert_eq!(t.text, "sTA");
        assert!(t.all(&[T::Dhatu, T::FlagAdeshadi]));
    }

    #[test]
    fn test_satva_blocked() {
        let t = check("zWivu~", "04.0004");
        assert_eq!(t.text, "zWiv");
        assert!(!t.has_tag(T::FlagAdeshadi));

        let t = check("zvazka~\\", "01.0105");
        assert_eq!(t.text, "zvazk");
        assert!(!t.has_tag(T::FlagAdeshadi));
    }

    #[test]
    fn test_natva() {
        let t = check("RI\\Y", "01.1049");
        assert_eq!(t.text, "nI");
        assert!(t.all(&[T::Dhatu, T::FlagAdeshadi]));
    }

    #[test]
    fn test_num_agama() {
        let t = check("vadi~\\", "01.0011");
        assert_eq!(t.text, "vand");
        assert!(t.has_tag(T::Dhatu));
    }
}
