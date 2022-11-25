use crate::sounds::s;
use crate::term::Term;

pub fn is_eka_ac(text: &str) -> bool {
    let ac = s("ac");
    let num_ac = text.chars().filter(|c| ac.contains_char(*c)).count();
    num_ac > 1
}

pub fn is_laghu(t: &Term) -> bool {
    match t.antya() {
        Some('a' | 'i' | 'u' | 'f' | 'x') => true,
        _ => false
    }
}

pub fn is_guru(t: &Term) -> bool {
    !is_laghu(t)
}
