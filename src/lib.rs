#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub use crate::ashtadhyayi::{Ashtadhyayi, AshtadhyayiBuilder};
pub use crate::prakriya::{Prakriya, Rule, RuleChoice, Step};

mod abhyasasya;
mod ac_sandhi;
mod angasya;
mod ardhadhatuka;
mod ashtadhyayi;
mod asiddhavat;
mod atidesha;
mod atmanepada;
mod char_view;
mod dhatu_gana;
mod dhatu_karya;
mod dvitva;
mod filters;
mod guna_vrddhi;
mod it_agama;
mod it_samjna;
mod la_karya;
mod operators;
mod samjna;
mod samprasarana;
mod sanadi;
mod stem_gana;
mod sup_adesha;
mod sup_karya;
mod tin_pratyaya;
mod tripadi;
mod vikarana;

pub mod args;
pub mod dhatupatha;
mod prakriya;
mod sounds;
mod tag;
mod term;
