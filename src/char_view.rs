use crate::prakriya::Prakriya;
use crate::term::Term;

/// Gets the term corresponding to character `i` of the current prakriya.
pub fn get_at(p: &mut Prakriya, index: usize) -> Option<&Term> {
    let mut cur = 0;
    for t in p.terms() {
        let delta = t.text.len();
        if (cur..cur + delta).contains(&index) {
            return Some(t);
        } else {
            cur += delta;
        }
    }
    None
}

/// Replaces character `i` of the current prakriya with the given substitute.
pub fn set_at(p: &mut Prakriya, index: usize, substitute: &str) {
    let mut cur = 0;
    for t in p.terms_mut() {
        let delta = t.text.len();
        if (cur..cur + delta).contains(&index) {
            let t_offset = index - cur;
            t.text = String::from(&t.text[..t_offset]) + substitute + &t.text[t_offset + 1..];
            return;
        } else {
            cur += delta;
        }
    }
}

/// Applies a sound-based rule to the given prakriya.
pub fn char_rule(
    p: &mut Prakriya,
    filter: impl Fn(&mut Prakriya, &str, usize) -> bool,
    operator: impl Fn(&mut Prakriya, &str, usize),
) {
    loop {
        let text = p.text();
        let mut applied_rule = false;

        for i in 0..text.len() {
            if filter(p, &text, i) {
                operator(p, &text, i);
                applied_rule = true;
                break;
            }
        }

        if !applied_rule {
            break;
        }
    }
}

pub fn xy(inner: impl Fn(char, char) -> bool) -> impl Fn(&mut Prakriya, &str, usize) -> bool {
    move |_, text, i| {
        let x = text.as_bytes().get(i);
        let y = text.as_bytes().get(i + 1);
        let (x, y) = match (x, y) {
            (Some(a), Some(b)) => (*a as char, *b as char),
            _ => return false,
        };
        inner(x, y)
    }
}

/// Applies a sound-based rule to the given prakriya.
pub fn char_rule_legacy(
    p: &mut Prakriya,
    filter: impl Fn(&mut Prakriya, char, char, usize, usize) -> bool,
    operator: impl Fn(&mut Prakriya, char, char, usize, usize),
) {
    char_rule(
        p,
        |p, text, i| {
            let j = i + 1;
            let x = text.as_bytes().get(i);
            let y = text.as_bytes().get(i + 1);
            let (x, y) = match (x, y) {
                (Some(a), Some(b)) => (*a as char, *b as char),
                _ => return false,
            };
            filter(p, x, y, i, j)
        },
        |p, text, i| {
            let j = i + 1;
            let x = text.as_bytes().get(i);
            let y = text.as_bytes().get(i + 1);
            let (x, y) = match (x, y) {
                (Some(a), Some(b)) => (*a as char, *b as char),
                _ => return,
            };
            operator(p, x, y, i, j);
        },
    );
}

pub fn xy2(
    inner: impl Fn(char, char) -> bool,
) -> impl Fn(&mut Prakriya, char, char, usize, usize) -> bool {
    move |_, x, y, _, _| inner(x, y)
}
