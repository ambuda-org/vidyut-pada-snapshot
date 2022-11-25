use crate::constants::Tag as T;
use crate::it_samjna;
use crate::prakriya::Prakriya;
use crate::sounds::s;
use crate::term::Term;

// Substitution
// ============

/// Replaces the first sound in the given term.
pub fn adi(t: &mut Term, sub: &str) {
    if let Some(c) = t.adi() {
        if c.to_string() != sub {
            t.text = String::from(sub) + &t.text[1..];
        }
    }
}

/// Replaces the last sound in the given term.
pub fn antya(t: &mut Term, sub: &str) {
    if let Some(c) = t.antya() {
        if c.to_string() != sub {
            let n = t.text.len();
            t.text = String::from(&t.text[..n - 1]) + sub;
        }
    }
}

/// Replaces the penultimate sound in the given term.
pub fn upadha(t: &mut Term, sub: &str) {
    if let Some(c) = t.upadha() {
        if c.to_string() != sub {
            let n = t.text.len();
            t.text = String::from(&t.text[..n - 2]) + sub + &t.text[n - 1..];
        }
    }
}

pub fn mit(t: &mut Term, sub: &str) {
    let text = &t.text;
    if let Some(i) = text.rfind(|c| s("ac").contains_char(c)) {
        t.text = String::from(&text[..=i]) + sub + &text[i + 1..];
    }
}

pub fn ti(t: &mut Term, sub: &str) {
    let text = &t.text;
    if let Some(i) = text.rfind(|c| s("ac").contains_char(c)) {
        t.text = String::from(&text[..i]) + sub;
    }
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


// Lopa
// ====

/// Delete the text in the given term.
fn lopa(t: &mut Term) {
    t.text = "".to_string();
}

/// Delete the text in the given term through `लुक्`.
pub fn luk(t: &mut Term) {
    lopa(t);
    t.add_tag(T::Luk);
}

/// Deletes the text in the given term through `श्लु`.
pub fn slu(t: &mut Term) {
    lopa(t);
    t.add_tag(T::Slu);
}

/// Deletes the text in the given term through `लुप्`.
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

pub fn none(_t: &mut Term) {}

pub fn t(i: usize, f: impl Fn(&mut Term)) -> impl Fn(&mut Prakriya) {
    move |p| {
        if let Some(t) = p.get_mut(i) {
            f(t);
        }
    }
}

pub fn add_tag(i: usize, tag: T) -> impl Fn(&mut Prakriya) {
    move |p| {
        if let Some(t) = p.get_mut(i) {
            t.add_tag(tag);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::term::Term;

    #[test]
    fn test_adi() {
        let mut t = Term::make_text("ji");
        adi(&mut t, "g");
        assert_eq!(t.text, "gi");
    }

    #[test]
    fn test_antya() {
        let mut t = Term::make_text("ti");
        antya(&mut t, "");
        assert_eq!(t.text, "t");
    }

    #[test]
    fn test_upadha() {
        let mut t = Term::make_text("sPur");
        upadha(&mut t, "A");
        assert_eq!(t.text, "sPAr");
    }

    #[test]
    fn test_mit() {
        let mut t = Term::make_text("vid");
        mit(&mut t, "n");
        assert_eq!(t.text, "vind");
    }

    #[test]
    fn test_ti() {
        let mut t = Term::make_text("AtAm");
        ti(&mut t, "e");
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
