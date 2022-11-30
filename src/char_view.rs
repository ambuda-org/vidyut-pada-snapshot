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
    operator: impl Fn(&mut Prakriya, &str, usize) -> bool,
) {
    let mut counter = 0;
    loop {
        let text = p.text();
        let mut changed_text = false;

        for i in 0..text.len() {
            if filter(p, &text, i) {
                changed_text = operator(p, &text, i);
                // Once the text has changed, our indices need to be reset. So, break the loop and
                // try again.
                if changed_text {
                    break;
                }
            }
        }

        if !changed_text {
            break;
        }

        counter += 1;
        if counter > 10 {
            panic!("Possible infinite loop: {:?}", p.history());
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

pub fn xyz(
    inner: impl Fn(char, char, char) -> bool,
) -> impl Fn(&mut Prakriya, &str, usize) -> bool {
    move |_, text, i| {
        let bytes = text.as_bytes();
        let x = bytes.get(i);
        let y = bytes.get(i + 1);
        let z = bytes.get(i + 2);
        let (x, y, z) = match (x, y, z) {
            (Some(a), Some(b), Some(c)) => (*a as char, *b as char, *c as char),
            _ => return false,
        };
        inner(x, y, z)
    }
}
