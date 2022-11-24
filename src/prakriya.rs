use crate::constants::Tag;
use crate::term::Term;
use std::collections::{HashMap, HashSet};

pub type Rule = &'static str;

/// Represents a step of the derivation.
pub struct Step {
    /// The rule that produced the current step.
    rule: Rule,
    /// Output for the current step.
    state: Vec<String>,
}

#[derive(Eq, PartialEq)]
pub enum RuleOption {
    /// Allow use of the given rule.
    Allow,
    /// Ignore the given rule and treat it as if it was never defined.
    Ignore,
}

pub enum RuleDecision {
    /// Whether a rule was used during the derivation.
    Accepted,
    /// Whether a rule was declined during the derivation.
    Declined,
}

pub struct Prakriya {
    terms: Vec<Term>,
    tags: HashSet<Tag>,
    steps: Vec<Step>,
    options_config: HashMap<Rule, RuleOption>,
    rule_decisions: Vec<(Rule, RuleDecision)>,
}

impl Prakriya {
    // Constructors

    pub fn new() -> Self {
        Prakriya {
            terms: Vec::new(),
            tags: HashSet::new(),
            steps: Vec::new(),
            options_config: HashMap::new(),
            rule_decisions: Vec::new(),
        }
    }

    pub fn from_terms(terms: &[Term]) -> Self {
        let mut p = Prakriya::new();
        p.terms = terms.to_vec();
        p
    }

    pub fn terms(&mut self) -> &mut Vec<Term> {
        &mut self.terms
    }

    pub fn get(&self, i: usize) -> Option<&Term> {
        self.terms.get(i)
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut Term> {
        self.terms.get_mut(i)
    }

    // Test

    pub fn has(&self, index: usize, filter: impl Fn(&Term) -> bool) -> bool {
        if let Some(t) = self.get(index) {
            filter(t)
        } else {
            false
        }
    }

    pub fn set(&mut self, index: usize, operator: impl Fn(&mut Term)) {
        if let Some(t) = self.get_mut(index) {
            operator(t);
        }
    }

    /// Apply the given rule.
    pub fn term_rule(
        &mut self,
        code: Rule,
        index: usize,
        filter: impl Fn(&Term) -> bool,
        operator: impl Fn(&mut Term),
    ) -> bool {
        self.rule(code, |p| p.has(index, &filter), |p| p.set(index, &operator))
    }

    /// Applies the given rule.
    pub fn rule(
        &mut self,
        code: Rule,
        filter: impl Fn(&Prakriya) -> bool,
        operator: impl Fn(&mut Prakriya),
    ) -> bool {
        if filter(self) {
            operator(self);
            self.step(code);
            true
        } else {
            false
        }
    }

    // Term mutators

    pub fn insert_before(&mut self, i: usize, t: Term) {
        self.terms.insert(i, t);
    }

    pub fn insert_after(&mut self, i: usize, t: Term) {
        self.terms.insert(i+1, t);
    }

    pub fn push(&mut self, t: Term) {
        self.terms.push(t);
    }

    pub fn text(&self) -> String {
        self.terms.iter().fold(String::new(), |a, b| a + &b.text)
    }

    pub fn add_tag(&mut self, t: Tag) {
        self.tags.insert(t);
    }

    pub fn is_allowed(&mut self, r: Rule) -> bool {
        *self.options_config.get(r).unwrap_or(&RuleOption::Allow) == RuleOption::Allow
    }

    pub fn decline(&mut self, r: Rule) {
        self.rule_decisions.push((r, RuleDecision::Declined));
    }

    pub fn find_first(&self, tag: Tag) -> Option<usize> {
        for (i, t) in self.terms.iter().enumerate() {
            if t.has_tag(tag) {
                return Some(i);
            }
        }
        None
    }

    pub fn find_last(&self, tag: &Tag) -> Option<(usize, &Term)> {
        for (i, t) in self.terms.iter().enumerate().rev() {
            if t.has_tag(*tag) {
                return Some((i, t));
            }
        }
        None
    }

    pub fn find_all<'a>(&'a self, tag: &'a Tag) -> impl Iterator<Item = &'a Term> {
        self.terms.iter().filter(|t| t.has_tag(*tag))
    }

    pub fn set_options_config(&mut self, o: HashMap<Rule, RuleOption>) {
        self.options_config = o;
    }

    pub fn all(&self, tags: &[Tag]) -> bool {
        tags.iter().all(|t| self.tags.contains(t))
    }

    pub fn any(&self, tags: &[Tag]) -> bool {
        tags.iter().any(|t| self.tags.contains(t))
    }

    pub fn add_tags(&mut self, tags: &[Tag]) {
        self.tags.extend(tags)
    }

    pub fn step(&mut self, rule: Rule) {
        self.steps.push(Step {
            rule,
            state: self.terms.iter().map(|x| x.text.clone()).collect(),
        })
    }
}

impl Default for Prakriya {
    fn default() -> Self {
        Prakriya::new()
    }
}
