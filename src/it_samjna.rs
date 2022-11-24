//! `it_samjna`
//! ===========
//! (1.3.2 - 1.3.9)
//!
//! The most "core" prakaraṇa is the it-saṁjñā-prakaraṇa, which identifies remove different `it`
//! sounds from an upadeśa. Most derivations use this prakaraṇa at least once.
use crate::constants::Tag as T;
use crate::prakriya::Prakriya;
use crate::sounds::{s, SoundSet};
use crate::term::Term;
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;


fn make_re_anunasika_ac() -> Regex {
    let s_ac = s("ac").items().join("");
    Regex::new(&format!("([{s_ac}]~[\\\\^]?)")).unwrap()
}

// 1.3.2 - 1.3.9
pub fn run(p: &mut Prakriya, i: usize) -> Result<(), Box<dyn Error>> {
    lazy_static! {
        // FIXME: find a better approach for `s`.
        static ref AC: SoundSet = s("ac");
        static ref HAL: SoundSet = s("hal");
        static ref TUSMA: SoundSet = s("tu~ s m");
        static ref CUTU: SoundSet = s("cu~ wu~");
        static ref LASHAKU: SoundSet = s("l S ku~");
        static ref RE_ANUNASIKA_AC: Regex = make_re_anunasika_ac();
    }

    let upadesha = match p.get(i) {
        Some(t) => match &t.u {
            Some(x) => x.clone(),
            None => return Ok(()),
        },
        None => return Ok(()),
    };
    // Wrap the upadesha text so that we can call `antya`, etc. against it.
    let u = Term::make_upadesha(&upadesha);

    // Varttika: `i~r` is its own it.
    let mut irit = false;
    if let Some(t) = p.get_mut(i) {
        if let Some(prefix) = t.text.strip_suffix("i~r") {
            t.text = prefix.to_string();
            t.add_tag(T::irit);
            irit = true;
        } else if let Some(prefix) = t.text.strip_suffix("i~^r") {
            t.text = prefix.to_string();
            t.add_tags(&[T::irit, T::svaritet]);
            irit = true;
        }
    }

    // 1.3.2 उपदेशे ऽजनुनासिक इत्
    if let Some(t) = p.get_mut(i) {
        let mut tags = vec![];
        for m in RE_ANUNASIKA_AC.find_iter(&t.text) {
            let s = m.as_str();
            if s.contains('\\') {
                tags.push(T::anudattet);
            } else if s.contains('^') {
                tags.push(T::svaritet);
            }
            tags.push(T::parse_it(&s[0..1])?);
        }
        t.text = RE_ANUNASIKA_AC.replace_all(&t.text, "").to_string();
        t.add_tags(&tags);
        p.step("1.3.2");
    }

    // Also handle general anudatta/svarita.
    if let Some(t) = p.get_mut(i) {
        if t.text.contains('\\') {
            t.add_tag(T::Anudatta);
        }
        if t.text.contains('^') {
            t.add_tag(T::Svarita);
        }
        t.text = t.text.replace('\\', "").replace('^', "");
    }

    if let Some(t) = p.get_mut(i) {
        if u.has_antya(&HAL) && !irit {
            let vibhaktau_tusmah = t.has_tag(T::Vibhakti) && u.has_antya(&TUSMA);
            if !vibhaktau_tusmah {
                t.add_tag(T::parse_it(&u.antya().unwrap().to_string())?);
                let n = t.text.len();
                t.text = t.text[..n - 1].to_string();
                p.step("1.3.3");
            } else {
                p.step("1.3.4");
            }
        }
    }

    if let Some(t) = p.get_mut(i) {
        let mut matched = false;
        for (it, tag) in [("Yi", T::YIt), ("wu", T::wvit), ("qu", T::qvit)] {
            if let Some(prefix) = t.text.strip_prefix(it) {
                t.text = prefix.to_string();
                t.add_tag(tag);
            }
            matched = true;
        }
        if matched {
            p.step("1.3.5");
        }
    }

    if let Some(t) = p.get_mut(i) {
        if t.has_tag(T::Pratyaya) {
            if u.text.starts_with('z') {
                t.add_tag(T::parse_it(&u.adi().unwrap().to_string())?);
                t.text = t.text[1..].to_string();
                p.step("1.3.6")
            } else if u.has_adi(&CUTU) {
                // The sounds C, J, W, and Q are replaced later in the grammar.
                // If we substitute them now, those rules will become vyartha.
                if !u.has_adi(&s("C J W Q")) {
                    t.add_tag(T::parse_it(&u.adi().unwrap().to_string())?);
                    t.text = t.text[1..].to_string();
                }
                p.step("1.3.7");
            } else if !t.has_tag(T::Taddhita) && t.has_adi(&LASHAKU) {
                // Keep the first "l" of the lakAras.
                // Otherwise, rule 3.4.77 will become vyartha.
                let lakara = [
                    "la~w", "li~w", "lu~w", "lf~w", "le~w", "lo~w", "la~N", "li~N", "lu~N", "lf~N",
                ];
                if !lakara.contains(&u.text.as_str()) {
                    t.add_tag(T::parse_it(&u.adi().unwrap().to_string())?);
                    t.text = t.text[1..].to_string();
                    p.step("1.3.8");
                }
            }
        }
    }

    if let Some(t) = p.get(i) {
        if t.text != u.text {
            p.step("1.3.9")
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(t: Term) -> Term {
        let mut p = Prakriya::new();
        p.push(t);
        run(&mut p, 0).unwrap();
        p.get(0).unwrap().clone()
    }

    #[test]
    fn test_no_upadesha() {
        let t = check(Term::make_text("buD"));
        assert_eq!(t.text, "buD");
    }

    #[test]
    fn test_common() {
        let tests = [
            ("i", "i", vec![]),
            ("df\\Si~r", "dfS", vec![T::irit, T::Anudatta]),
            ("ga\\mx~", "gam", vec![T::xdit]),
            ("vftu~\\", "vft", vec![T::udit, T::anudattet]),
            ("qukfY", "kf", vec![T::qvit, T::Yit]),
            (
                "qupa\\ca~^z",
                "pac",
                vec![T::qvit, T::Anudatta, T::adit, T::svaritet, T::zit],
            ),
        ];

        for (raw, text, tags) in tests {
            let t = check(Term::make_upadesha(raw));
            assert_eq!(t.text, text);
            assert!(t.all(&tags));
        }
    }

    #[test]
    fn test_vibhakti() {
        let tests = [
            ("su~", "s", vec![T::udit]),
            ("tip", "ti", vec![T::pit]),
            ("t", "t", vec![]),
            ("n", "n", vec![]),
            ("mas", "mas", vec![]),
            ("AtAm", "AtAm", vec![]),
        ];

        for (raw, text, tags) in tests {
            let mut start = Term::make_upadesha(raw);
            start.add_tag(T::Vibhakti);
            let t = check(start);

            assert_eq!(t.text, text);
            assert!(t.all(&tags));
        }
    }

    #[test]
    fn test_pratyaya() {
        let tests = [
            ("kta", "ta", vec![T::Pratyaya, T::kit]),
            ("Ric", "i", vec![T::Pratyaya, T::Rit, T::cit]),
            ("la~w", "l", vec![T::Pratyaya, T::adit, T::wit]),
        ];
        for (raw, text, tags) in tests {
            let mut start = Term::make_upadesha(raw);
            start.add_tag(T::Pratyaya);
            let t = check(start);

            assert_eq!(t.text, text, "{text}");
            assert!(t.all(&tags), "Missing one or more of `{tags:?}`");
        }
    }
}
