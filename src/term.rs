use crate::constants::Tag;
use crate::sounds::SoundSet;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Term {
    pub u: Option<String>,
    pub text: String,
    pub tags: HashSet<Tag>,
    pub gana: Option<i32>,
    pub number: Option<i32>,
    pub lakshana: Vec<String>,
}

impl Term {
    pub fn make_upadesha(s: &str) -> Self {
        Term {
            u: Some(s.to_string()),
            text: s.to_string(),
            tags: HashSet::new(),
            gana: None,
            number: None,
            lakshana: Vec::new(),
        }
    }

    pub fn make_text(s: &str) -> Self {
        Term {
            u: None,
            text: s.to_string(),
            tags: HashSet::new(),
            gana: None,
            number: None,
            lakshana: Vec::new(),
        }
    }

    pub fn make_dhatu(s: &str, gana: i32, number: i32) -> Self {
        let mut t = Term::make_upadesha(s);
        t.gana = Some(gana);
        t.number = Some(number);
        t
    }

    pub fn make_agama(s: &str) -> Self {
        let mut t = Term::make_upadesha(s);
        t.add_tag(Tag::Agama);
        t
    }

    // Sound selection

    pub fn adi(&self) -> Option<char> {
        self.text.chars().next()
    }

    pub fn antya(&self) -> Option<char> {
        self.text.chars().rev().next()
    }

    pub fn upadha(&self) -> Option<char> {
        self.text.chars().rev().nth(1)
    }

    pub fn get(&self, i: usize) -> Option<char> {
        let n = self.text.len();
        if i < n {
            Some(self.text.as_bytes()[i] as char)
        } else {
            None
        }
    }

    // Sound properties

    fn has_sound_in_set(&self, c: Option<char>, set: &SoundSet) -> bool {
        match c {
            Some(c) => set.contains_char(c),
            None => false,
        }
    }

    pub fn has_adi(&self, set: &SoundSet) -> bool {
        self.has_sound_in_set(self.adi(), set)
    }

    pub fn has_antya(&self, set: &SoundSet) -> bool {
        self.has_sound_in_set(self.antya(), set)
    }

    pub fn has_upadha(&self, set: &SoundSet) -> bool {
        self.has_sound_in_set(self.upadha(), set)
    }

    pub fn has_u(&self, s: &str) -> bool {
        match &self.u {
            Some(u) => u == s,
            None => false,
        }
    }

    pub fn has_u_in(&self, items: &[&str]) -> bool {
        match &self.u {
            Some(u) => items.contains(&u.as_str()),
            None => false,
        }
    }

    pub fn has_lakshana(&self, u: &str) -> bool {
        self.lakshana.iter().any(|s| s == u)
    }

    pub fn has_any_lakshana(&self, u: &[&str]) -> bool {
        self.lakshana.iter().any(|s| u.contains(&s.as_str()))
    }

    pub fn has_text(&self, items: &[&str]) -> bool {
        items.contains(&self.text.as_str())
    }

    pub fn starts_with_any(&self, terms: &[&str]) -> bool {
        terms.iter().any(|t| self.text.starts_with(t))
    }

    // Tags

    pub fn all(&self, tags: &[Tag]) -> bool {
        tags.iter().all(|t| self.tags.contains(t))
    }

    pub fn any(&self, tags: &[Tag]) -> bool {
        tags.iter().any(|t| self.tags.contains(t))
    }

    pub fn has_tag(&self, tag: Tag) -> bool {
        self.tags.contains(&tag)
    }

    // Mutators

    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.insert(tag);
    }

    pub fn add_tags(&mut self, tags: &[Tag]) {
        self.tags.extend(tags)
    }

    pub fn remove_tag(&mut self, tag: Tag) {
        self.tags.remove(&tag);
    }

    pub fn remove_tags(&mut self, tags: &[Tag]) {
        for t in tags {
            self.tags.remove(t);
        }
    }
}

/// An abstra
pub struct TermView<'a> {
    terms: &'a Vec<Term>,
    start: usize,
    end: usize,
}

impl<'a> TermView<'a> {
    pub fn new(terms: &'a Vec<Term>, start: usize) -> Option<Self> {
        if start >= terms.len() {
            return None;
        }

        let mut end = start;
        for (i, t) in terms.iter().enumerate().filter(|(i, _)| *i >= start) {
            if !t.has_tag(Tag::Agama) {
                end = i;
                break;
            }
        }
        Some(TermView { terms, start, end })
    }

    pub fn slice(&self) -> &[Term] {
        &self.terms[self.start..=self.end]
    }

    pub fn adi(&self) -> Option<char> {
        for t in self.slice() {
            match t.adi() {
                Some(c) => return Some(c),
                None => continue
            }
        }
        None
    }

    pub fn antya(&self) -> Option<char> {
        for t in self.slice().iter().rev() {
            match t.antya() {
                Some(c) => return Some(c),
                None => continue
            }
        }
        None
    }

    pub fn has_tag(&self, tag: Tag) -> bool {
        self.slice().iter().any(|t| t.has_tag(tag))
    }

    pub fn all(&self, tags: &[Tag]) -> bool {
        for tag in tags {
            if self.slice().iter().any(|t| t.has_tag(*tag)) {
                continue;
            }
            return false; 
        }
        true
    }

    pub fn any(&self, tags: &[Tag]) -> bool {
        tags.iter().any(|tag| {
           self.slice().iter().any(|t| t.has_tag(*tag))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gam() -> Term {
        let mut t = Term::make_upadesha("gamx~");
        t.text = "gam".to_string();
        t
    }

    #[test]
    fn test_make_upadesha() {
        let t = Term::make_upadesha("Satf");
        assert_eq!(t.u, Some("Satf".to_string()));
        assert_eq!(t.text, "Satf");
    }

    #[test]
    fn test_sound_selectors() {
        let t = gam();

        assert_eq!(t.adi(), Some('g'));
        assert_eq!(t.upadha(), Some('a'));
        assert_eq!(t.antya(), Some('m'));

        assert_eq!(t.get(0), Some('g'));
        assert_eq!(t.get(1), Some('a'));
        assert_eq!(t.get(2), Some('m'));
        assert_eq!(t.get(3), None);
    }
}
