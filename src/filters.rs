/*!
Components for creating filters.

We model rules as having two parts: a `filter` that determines whether the rule can apply to some
*prakriya* and an `operator` that changes the *prakriya*. This module contains useful standalone
filters and various utilities for working with filters in the rest of the system.

*/
use crate::constants::Tag as T;
use crate::sounds as al;
use crate::sounds::{s, SoundSet};
use crate::term::Term;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: SoundSet = s("ac");
    static ref HAL: SoundSet = s("hal");
}

/// Returns whether the given term has exactly one vowel sound.
pub fn is_eka_ac(t: &Term) -> bool {
    let num_ac = t.text.chars().filter(|c| AC.contains_char(*c)).count();
    num_ac == 1
}

/// Returns whether the term begins with a conjunct consonant.
pub fn is_samyogadi(t: &Term) -> bool {
    al::is_samyogadi(&t.text)
}

/// Returns whether the term ends in a conjunct consonant.
pub fn is_samyoganta(t: &Term) -> bool {
    al::is_samyoganta(&t.text)
}

/// Returns whether the term is a pratyaya with exactly one sound.
pub fn is_aprkta(t: &Term) -> bool {
    t.has_tag(T::Pratyaya) && t.text.len() == 1
}

/// Returns whether the last syllable of `t` is or could be laghu.
pub fn is_laghu(t: &Term) -> bool {
    // 1.4.10 hrasvaM laghu
    // 1.4.11 saMyoge guru
    // 1.4.12 dIrghaM ca
    if let Some(c) = t.antya() {
        if al::is_ac(c) {
            al::is_hrasva(c)
        } else {
            // upadha is hrasva --> laghu
            // upadha is dirgha --> guru
            // upadha is hal --> guru (samyoga)
            // upadha is missing --> laghu
            t.upadha().map(al::is_hrasva).unwrap_or(false)
            // HACK for C, which will always become cC (= guru).
            && c != 'C'
        }
    } else {
        false
    }
}

/// Returns whether the last syllable of `t` is guru.
pub fn is_guru(t: &Term) -> bool {
    !is_laghu(t)
}

pub fn is_hrasva(t: &Term) -> bool {
    !is_dirgha(t)
}

pub fn is_dirgha(t: &Term) -> bool {
    match t.antya() {
        Some(c) => al::is_dirgha(c),
        None => false,
    }
}

pub fn ends_with(sub: &'static str) -> impl Fn(&Term) -> bool {
    move |t| t.text.ends_with(sub)
}

pub fn empty(t: &Term) -> bool {
    t.text.is_empty()
}

/// Returns whether the term has the given `tag`.
pub fn tag(tag: T) -> impl Fn(&Term) -> bool {
    move |t| t.has_tag(tag)
}

/// Returns whether the term has the given `tag`.
pub fn tag_in(tags: &'static [T]) -> impl Fn(&Term) -> bool {
    move |t| t.has_tag_in(tags)
}

/// Returns whether the term is a dhatu.
pub fn dhatu(t: &Term) -> bool {
    t.has_tag(T::Dhatu)
}

/// Returns whether the term is an Atmanepada pratyaya.
pub fn atmanepada(t: &Term) -> bool {
    t.has_tag(T::Atmanepada)
}

/// Returns whether the term is a Sarvadhatuka pratyaya.
pub fn sarvadhatuka(t: &Term) -> bool {
    t.has_tag(T::Sarvadhatuka)
}

/// Returns whether the term is a sup pratyaya.
pub fn sup(t: &Term) -> bool {
    t.has_tag(T::Sup)
}

/// Returns whether the term's text is exactly `x`.
pub fn text(x: &'static str) -> impl Fn(&Term) -> bool {
    move |t| t.text == x
}

/// Returns whether the term's text is contained in `xs`.
pub fn text_in(xs: &'static [&str]) -> impl Fn(&Term) -> bool {
    move |t| t.has_text_in(xs)
}

pub fn lakshana(text: &'static str) -> impl Fn(&Term) -> bool {
    move |t| t.has_lakshana(text)
}

pub fn lakshana_in(xs: &'static [&str]) -> impl Fn(&Term) -> bool {
    move |t| t.has_lakshana_in(xs)
}

/// Returns whether the term's upadesha is exactly `x`.
pub fn u(u: &'static str) -> impl Fn(&Term) -> bool {
    move |t| t.has_u(u)
}

/// Returns whether the term's upadesha is contained in `xs`.
pub fn u_in(us: &'static [&str]) -> impl Fn(&Term) -> bool {
    move |t| t.has_u_in(us)
}

pub fn is_asti(t: &Term) -> bool {
    t.has_u("asa~") && t.has_gana(2)
}

pub fn is_it_agama(t: &Term) -> bool {
    t.has_u("iw") && t.has_tag(T::Agama)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_laghu() {
        let text_is_laghu = |s| {
            let t = Term::make_text(s);
            is_laghu(&t)
        };
        assert!(text_is_laghu("i"));
        assert!(text_is_laghu("vid"));
        assert!(!text_is_laghu("BU"));
        assert!(!text_is_laghu("uC"));
        assert!(!text_is_laghu("IS"));
    }
}
