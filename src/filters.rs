use crate::constants::Tag as T;
use crate::prakriya::Prakriya;
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

pub fn is_aprkta(t: &Term) -> bool {
    t.has_tag(T::Pratyaya) && t.text.len() == 1
}

pub fn is_knit(t: &Term) -> bool {
    t.any(&[T::kit, T::Nit])
}

pub fn is_laghu(t: &Term) -> bool {
    matches!(t.antya(), Some('a' | 'i' | 'u' | 'f' | 'x'))
}

pub fn is_hrasva(t: &Term) -> bool {
    is_laghu(t)
}

pub fn is_guru(t: &Term) -> bool {
    !is_laghu(t)
}

//

pub fn ends_with(sub: &'static str) -> impl Fn(&Term) -> bool {
    move |t| t.text.ends_with(sub)
}

pub fn has_tag(tag: T) -> impl Fn(&Term) -> bool {
    move |t| t.has_tag(tag)
}

pub fn text(text: &'static str) -> impl Fn(&Term) -> bool {
    move |t| t.text == text
}

pub fn lakshana(text: &'static str) -> impl Fn(&Term) -> bool {
    move |t| t.has_lakshana(text)
}

pub fn lakshana_in(xs: &'static [&str]) -> impl Fn(&Term) -> bool {
    move |t| t.has_any_lakshana(xs)
}

pub fn u(u: &'static str) -> impl Fn(&Term) -> bool {
    move |t| t.has_u(u)
}

pub fn u_in(us: &'static [&str]) -> impl Fn(&Term) -> bool {
    move |t| t.has_u_in(us)
}
