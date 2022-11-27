/*!
Components for creating filters.

We model rules as having two parts: a `filter` that determines whether the rule can apply to some
*prakriya* and an `operator` that changes the *prakriya*. This module contains useful standalone
filters and various utilities for working with filters in the rest of the system.

*/
use crate::constants::Tag as T;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::sounds::{s, SoundSet};
use crate::term::Term;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: SoundSet = s("ac");
    static ref HAL: SoundSet = s("hal");
}

pub fn t(i: usize, f: impl Fn(&Term) -> bool) -> impl Fn(&mut Prakriya) -> bool {
    move |p| match p.get(i) {
        Some(t) => f(t),
        None => false,
    }
}

/// Returns whether the given term has exactly one vowel sound.
pub fn is_eka_ac(t: &Term) -> bool {
    let num_ac = t.text.chars().filter(|c| AC.contains_char(*c)).count();
    num_ac == 1
}

/// Returns whether the term begins with a conjunct consonant.
pub fn is_samyogadi(t: &Term) -> bool {
    let mut chars = t.text.chars();
    HAL.contains_opt(chars.next()) && HAL.contains_opt(chars.next())
}

/// Returns whether the term ends in a conjunct consonant.
pub fn is_samyoganta(t: &Term) -> bool {
    let mut chars = t.text.chars().rev();
    HAL.contains_opt(chars.next()) && HAL.contains_opt(chars.next())
}

/// Returns whether the term is a pratyaya with exactly one sound.
pub fn is_aprkta(t: &Term) -> bool {
    t.has_tag(T::Pratyaya) && t.text.len() == 1
}

pub fn is_knit(t: &Term) -> bool {
    t.any(&[T::kit, T::Nit])
}

pub fn is_laghu(t: &Term) -> bool {
    match t.antya() {
        Some(c) => al::is_hrasva(c),
        None => false,
    }
}

/// Returns whether the term ends in a *hrasva* vowel.
pub fn is_hrasva(t: &Term) -> bool {
    is_laghu(t)
}

/// Returns whether the term ends in a *guru* syllable.
pub fn is_guru(t: &Term) -> bool {
    !is_laghu(t)
}

pub fn ends_with(sub: &'static str) -> impl Fn(&Term) -> bool {
    move |t| t.text.ends_with(sub)
}

pub fn empty(t: &Term) -> bool {
    t.text.is_empty()
}

pub fn not_empty(t: &Term) -> bool {
    !t.text.is_empty()
}

/// Returns whether the term has the given `tag`.
pub fn tag(tag: T) -> impl Fn(&Term) -> bool {
    move |t| t.has_tag(tag)
}

/// Returns whether the term has the given `tag`.
pub fn tag_in(tags: &'static [T]) -> impl Fn(&Term) -> bool {
    move |t| t.any(tags)
}

/// Returns whether the term is an Atmanepada pratyaya.
pub fn atmanepada(t: &Term) -> bool {
    t.has_tag(T::Atmanepada)
}

/// Returns whether the term is a sup pratyaya.
pub fn sup(t: &Term) -> bool {
    t.has_tag(T::Sup)
}

/// Returns whether the term's text is exactly `x`.
pub fn antya(sounds: &'static str) -> impl Fn(&Term) -> bool {
    let sounds = s(sounds);
    move |t| sounds.contains_opt(t.antya())
}

/// Returns whether the term's text is exactly `x`.
pub fn text(x: &'static str) -> impl Fn(&Term) -> bool {
    move |t| t.text == x
}

/// Returns whether the term's text is contained in `xs`.
pub fn text_in(xs: &'static [&str]) -> impl Fn(&Term) -> bool {
    move |t| t.has_text(xs)
}

pub fn lakshana(text: &'static str) -> impl Fn(&Term) -> bool {
    move |t| t.has_lakshana(text)
}

pub fn lakshana_in(xs: &'static [&str]) -> impl Fn(&Term) -> bool {
    move |t| t.has_any_lakshana(xs)
}

/// Returns whether the term's upadesha is exactly `x`.
pub fn u(u: &'static str) -> impl Fn(&Term) -> bool {
    move |t| t.has_u(u)
}

/// Returns whether the term's upadesha is contained in `xs`.
pub fn u_in(us: &'static [&str]) -> impl Fn(&Term) -> bool {
    move |t| t.has_u_in(us)
}
