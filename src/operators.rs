/*!
Components for creating operators.

We model rules as having two parts: a `filter` that determines whether the rule can apply to some
*prakriya* and an `operator` that changes the *prakriya*. This module contains useful standalone
operators and various utilities for working with operators in the rest of the system.


# Types of operators

Broadly, our system has two kinds of operators: `Term` operators and `Prakriya` operators. A `Term`
operator accepts a single term and mutates it in some way, and a `Prakriya` operator does the same
for a `Prakriya`. Most of our operators are `Term` operators, but we can convert these operators to
`Prakriya` operators with the `t` function.


# Technical design

Generally, the functions here accept one or more static parameters then return `Term` operators as
closures. This approach gives us a terse and simple scheme for describing various operations, and
Rust's zero-cost abstractions ensure that there is no runtime penalty for juggling so many
closures.
*/
use crate::constants::Tag as T;
use crate::it_samjna;
use crate::prakriya::{Prakriya, Rule};
use crate::sounds::is_ac;
use crate::term::Term;

/// Wraps a `Term` operator and converts it to a `Prakriya` operator.
pub fn t(i: usize, f: impl Fn(&mut Term)) -> impl Fn(&mut Prakriya) {
    move |p| {
        if let Some(t) = p.get_mut(i) {
            f(t);
        }
    }
}

// Substitution
// ============

/// Replaces the first sound in the given term.
pub fn adi(sub: &str) -> impl Fn(&mut Term) + '_ {
    move |t| {
        let n = t.text.len();
        if n > 0 {
            t.text = String::from(sub) + &t.text[1..];
        }
    }
}

/// Replaces the last sound in the given term.
pub fn antya(sub: &str) -> impl Fn(&mut Term) + '_ {
    |t| {
        let n = t.text.len();
        if n > 0 {
            t.text = String::from(&t.text[..n - 1]) + sub;
        }
    }
}

pub fn set_upadha(text: &str, sub: &str) -> String {
    let n = text.len();
    String::from(&text[..n - 2]) + sub + &text[n - 1..]
}

/// Replaces the penultimate sound in the given term.
pub fn upadha(sub: &str) -> impl Fn(&mut Term) + '_ {
    |t| {
        if t.upadha().is_some() {
            t.text = set_upadha(&t.text, sub);
        }
    }
}

/// Inserts some text immediately after the term's last vowel:
///
/// > mid aco 'ntyāt paraḥ (1.1.47)
pub fn mit(sub: &'static str) -> impl Fn(&mut Term) {
    |t| {
        let text = &t.text;
        if let Some(i) = text.rfind(is_ac) {
            t.text = String::from(&text[..=i]) + sub + &text[i + 1..];
        }
    }
}

/// Replaces the `ti` region of the given term.
///
/// The `ti` region starts at the term's last vowel and continues to the end of the string:
///
/// > aco 'ntyādi ṭi (1.1.64)
pub fn ti(sub: &'static str) -> impl Fn(&mut Term) {
    move |t| {
        let text = &t.text;
        if let Some(i) = text.rfind(is_ac) {
            t.text = String::from(&text[..i]) + sub;
        }
    }
}

/// Replaces all of the text of the given term.
pub fn text(sub: &'static str) -> impl Fn(&mut Term) {
    |t| t.text = sub.to_string()
}

/// Replaces all of the text in the given term.
pub fn text2(rule: Rule, p: &mut Prakriya, i: usize, sub: &'static str) {
    p.op_term(rule, i, text(sub));
}

pub fn upadesha_no_it(p: &mut Prakriya, i: usize, sub: &str) {
    if let Some(t) = p.get_mut(i) {
        if let Some(u) = &t.u {
            t.lakshana.push(u.to_string());
        }
        t.u = Some(sub.to_string());
        t.text = sub.to_string();
    }
}

pub fn insert_agama_before(p: &mut Prakriya, i: usize, u: &str) {
    let agama = Term::make_agama(u);
    p.insert_before(i, agama);
}

pub fn insert_agama_after(p: &mut Prakriya, i: usize, u: &str) {
    let agama = Term::make_agama(u);
    p.insert_after(i, agama);
}

pub fn upadesha(p: &mut Prakriya, i: usize, sub: &str) {
    if let Some(t) = p.get_mut(i) {
        if let Some(u) = &t.u {
            t.lakshana.push(u.to_string());
        }
        t.u = Some(sub.to_string());
        t.text = sub.to_string();
        it_samjna::run(p, i).unwrap();
    }
}

pub fn upadesha_v2(rule: Rule, p: &mut Prakriya, i: usize, sub: &str) {
    if let Some(t) = p.get_mut(i) {
        if let Some(u) = &t.u {
            t.lakshana.push(u.to_string());
        }
        t.u = Some(sub.to_string());
        t.text = sub.to_string();
        p.step(rule);
        it_samjna::run(p, i).unwrap();
    }
}

/// Complex op
pub fn append_agama(rule: Rule, p: &mut Prakriya, i: usize, sub: &str) {
    let agama = Term::make_agama(sub);
    p.insert_after(i, agama);
    p.step(rule);
    it_samjna::run(p, i + 1).unwrap();
}

/// Complex op
pub fn adesha(rule: Rule, p: &mut Prakriya, i: usize, sub: &str) {
    if let Some(t) = p.get_mut(i) {
        if let Some(u) = &t.u {
            t.lakshana.push(u.to_string());
        }
        t.u = Some(sub.to_string());
        t.text = sub.to_string();
        p.step(rule);
        it_samjna::run(p, i).unwrap();
    }
}

pub fn yatha<'a>(needle: &str, old: &[&'a str], new: &[&'a str]) -> Option<&'a str> {
    for (i, o) in old.iter().enumerate() {
        if needle == *o {
            return Some(new[i]);
        }
    }
    None
}

pub fn upadesha_yatha(p: &mut Prakriya, i: usize, old: &[&str], new: &[&str]) {
    assert_eq!(old.len(), new.len());
    if let Some(t) = p.get_mut(i) {
        if let Some(u) = &t.u {
            t.lakshana.push(u.to_string());

            for (i_entry, x) in old.iter().enumerate() {
                if u == x {
                    t.set_upadesha(new[i_entry]);
                    it_samjna::run(p, i).unwrap();
                    return;
                }
            }
            panic!("Should not have reached here {:?} {:?} {:?}", old, new, t.u);
        }
    }
}

pub fn text_yatha(term: &mut Term, old: &[&str], new: &[&str]) {
    assert_eq!(old.len(), new.len());
    for (i, o) in old.iter().enumerate() {
        if term.text == *o {
            term.text = new[i].to_string();
            return;
        }
    }
}

// Lopa
// ====

/// Deletes all of the text in the given term.
pub fn lopa(t: &mut Term) {
    t.text = "".to_string();
}

/// Delete all of the text in the given term through *luk*.
pub fn luk(t: &mut Term) {
    lopa(t);
    t.add_tag(T::Luk);
}

/// Deletes all of the text in the given term through *ślu*.
pub fn slu(t: &mut Term) {
    lopa(t);
    t.add_tag(T::Slu);
}

/// Deletes all of the text in the given term through *lup*.
#[allow(unused)]
fn lup(t: &mut Term) {
    lopa(t);
    t.add_tag(T::Lup);
}

// Tags
// ====

/// Adds the given samjna.
pub fn samjna(t: &mut Term, tag: T) {
    t.add_tag(tag);
}

pub fn none(_: &mut Term) {}

pub fn add_tag(tag: T) -> impl Fn(&mut Term) {
    move |t| t.add_tag(tag)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::term::Term;

    #[test]
    fn test_adi() {
        let mut t = Term::make_text("ji");
        adi("g")(&mut t);
        assert_eq!(t.text, "gi");
    }

    #[test]
    fn test_antya() {
        let mut t = Term::make_text("ti");
        antya("")(&mut t);
        assert_eq!(t.text, "t");
    }

    #[test]
    fn test_upadha() {
        let mut t = Term::make_text("sPur");
        upadha("A")(&mut t);
        assert_eq!(t.text, "sPAr");
    }

    #[test]
    fn test_mit() {
        let mut t = Term::make_text("vid");
        mit("n")(&mut t);
        assert_eq!(t.text, "vind");
    }

    #[test]
    fn test_ti() {
        let mut t = Term::make_text("AtAm");
        ti("e")(&mut t);
        assert_eq!(t.text, "Ate");
    }

    #[test]
    fn test_lopa() {
        let mut t = Term::make_text("ti");
        lopa(&mut t);
        assert_eq!(t.text, "");
    }

    #[test]
    fn test_luk() {
        let mut t = Term::make_text("ti");
        luk(&mut t);
        assert_eq!(t.text, "");
        assert!(t.has_tag(T::Luk));
    }

    #[test]
    fn test_slu() {
        let mut t = Term::make_text("ti");
        slu(&mut t);
        assert_eq!(t.text, "");
        assert!(t.has_tag(T::Slu));
    }

    #[test]
    fn test_lup() {
        let mut t = Term::make_text("ti");
        lup(&mut t);
        assert_eq!(t.text, "");
        assert!(t.has_tag(T::Lup));
    }
}
