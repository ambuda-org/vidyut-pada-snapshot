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

#[derive(Default)]
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

    // Term accessors

    /// Returns all terms.
    pub fn terms(&mut self) -> &mut Vec<Term> {
        &mut self.terms
    }

    /// Returns a reference to the `Term` at the given index or `None` if the index is out of
    /// bounds.
    pub fn get(&self, i: usize) -> Option<&Term> {
        self.terms.get(i)
    }

    /// Returns a mutable reference to the `Term` at the given index or `None` if the index is out
    /// of bounds.
    pub fn get_mut(&mut self, i: usize) -> Option<&mut Term> {
        self.terms.get_mut(i)
    }

    /// Returns the index of the first `Term` that has the given tag or `None` if no such term
    /// exists.
    pub fn find_first(&self, tag: Tag) -> Option<usize> {
        for (i, t) in self.terms.iter().enumerate() {
            if t.has_tag(tag) {
                return Some(i);
            }
        }
        None
    }

    /// Returns the index of the last `Term` that has the given tag or `None` if no such term
    /// exists.
    pub fn find_last(&self, tag: Tag) -> Option<usize> {
        for (i, t) in self.terms.iter().enumerate().rev() {
            if t.has_tag(tag) {
                return Some(i);
            }
        }
        None
    }

    /// Returns all of the terms that have the given tag.
    pub fn find_all<'a>(&'a self, tag: &'a Tag) -> impl Iterator<Item = &'a Term> {
        self.terms.iter().filter(|t| t.has_tag(*tag))
    }

    // Filters

    /// Returns whether a term exists at `index` and matches the condition in `filter`.
    pub fn has(&self, index: usize, filter: impl Fn(&Term) -> bool) -> bool {
        if let Some(t) = self.get(index) {
            filter(t)
        } else {
            false
        }
    }

    pub fn all(&self, tags: &[Tag]) -> bool {
        tags.iter().all(|t| self.tags.contains(t))
    }

    pub fn any(&self, tags: &[Tag]) -> bool {
        tags.iter().any(|t| self.tags.contains(t))
    }

    pub fn has_tag(&self, tag: Tag) -> bool {
        self.tags.contains(&tag)
    }

    // Basic mutators

    pub fn add_tag(&mut self, t: Tag) {
        self.tags.insert(t);
    }

    pub fn add_tags(&mut self, tags: &[Tag]) {
        self.tags.extend(tags)
    }

    pub fn set(&mut self, index: usize, operator: impl Fn(&mut Term)) {
        if let Some(t) = self.get_mut(index) {
            operator(t);
        }
    }

    pub fn insert_before(&mut self, i: usize, t: Term) {
        self.terms.insert(i, t);
    }

    pub fn insert_after(&mut self, i: usize, t: Term) {
        self.terms.insert(i + 1, t);
    }

    /// Adds the given term to the end of the term list.
    pub fn push(&mut self, t: Term) {
        self.terms.push(t);
    }

    // Rule application

    /// Applies the given rule.
    pub fn term_rule(
        &mut self,
        code: Rule,
        index: usize,
        filter: impl Fn(&Term) -> bool,
        operator: impl Fn(&mut Term),
    ) -> bool {
        self.rule(code, |p| p.has(index, &filter), |p| p.set(index, &operator))
    }

    /// Applies the given operator.
    pub fn op(
        &mut self,
        code: Rule,
        operator: impl Fn(&mut Prakriya),
    ) -> bool {
        operator(self);
        self.step(code);
        true
    }

    /// Applies the given operator optionally.
    pub fn op_optional(
        &mut self,
        code: Rule,
        operator: impl Fn(&mut Prakriya),
    ) -> bool {
        if self.is_allowed(code) {
            operator(self);
            self.step(code);
            true
        } else {
            self.decline(code);
            false
        }
    }

    /// Applies the given rule.
    pub fn rule(
        &mut self,
        code: Rule,
        filter: impl Fn(&Prakriya) -> bool,
        operator: impl Fn(&mut Prakriya),
    ) -> bool {
        if filter(self) {
            self.op(code, operator)
        } else {
            false
        }
    }

    /// Applies the given rule optionally.
    pub fn optional(
        &mut self,
        code: Rule,
        filter: impl Fn(&Prakriya) -> bool,
        operator: impl Fn(&mut Prakriya),
    ) -> bool {
        if filter(self) {
            self.op_optional(code, operator)
        } else {
            false
        }
    }

    /// Add a rule to the history.
    pub fn step(&mut self, rule: Rule) {
        self.steps.push(Step {
            rule,
            state: self.terms.iter().map(|x| x.text.clone()).collect(),
        })
    }

    // Optional rules

    pub fn is_allowed(&mut self, r: Rule) -> bool {
        *self.options_config.get(r).unwrap_or(&RuleOption::Allow) == RuleOption::Allow
    }

    pub fn decline(&mut self, r: Rule) {
        self.rule_decisions.push((r, RuleDecision::Declined));
    }

    pub fn set_options_config(&mut self, o: HashMap<Rule, RuleOption>) {
        self.options_config = o;
    }

    // Final output

    pub fn text(&self) -> String {
        self.terms.iter().fold(String::new(), |a, b| a + &b.text)
    }
}
