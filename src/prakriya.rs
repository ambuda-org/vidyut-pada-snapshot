use crate::constants::Tag;
use crate::term::{Term, TermView};
use compact_str::CompactString;
use enumset::EnumSet;

/// A string code for some grammar rule. All rule codes are static strings.
pub type Rule = &'static str;

/// Represents a step of the derivation.
#[derive(Debug)]
pub struct Step {
    /// The rule that produced the current step.
    pub rule: Rule,
    /// Output for the current step.
    pub state: String,
}

#[derive(Clone, Copy, Debug)]
pub enum RuleChoice {
    /// Whether a rule was used during the derivation.
    Accept(Rule),
    /// Whether a rule was declined during the derivation.
    Decline(Rule),
}

#[derive(Default)]
pub struct Config {
    pub rule_choices: Vec<RuleChoice>,
    pub log_steps: bool,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Models a derivation.
#[derive(Default)]
pub struct Prakriya {
    terms: Vec<Term>,
    tags: EnumSet<Tag>,
    history: Vec<Step>,
    pub config: Config,
    rule_decisions: Vec<RuleChoice>,
}

impl Prakriya {
    // Constructors

    // Creates an empty prakriya.
    pub fn new() -> Self {
        Prakriya {
            terms: Vec::new(),
            tags: EnumSet::new(),
            history: Vec::new(),
            config: Config::new(),
            rule_decisions: Vec::new(),
        }
    }

    pub fn with_config(config: Config) -> Self {
        let mut p = Prakriya::new();
        p.config = config;
        p
    }

    // Term accessors

    pub fn rule_choices(&self) -> &Vec<RuleChoice> {
        &self.rule_decisions
    }

    pub fn history(&self) -> &Vec<Step> {
        &self.history
    }

    /// Returns all terms.
    pub fn terms(&self) -> &Vec<Term> {
        &self.terms
    }

    /// Returns all terms mutably.
    pub fn terms_mut(&mut self) -> &mut Vec<Term> {
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

    pub fn view(&self, i: usize) -> Option<TermView> {
        TermView::new(self.terms(), i)
    }

    pub fn find_first_where(&self, f: impl Fn(&Term) -> bool) -> Option<usize> {
        for (i, t) in self.terms.iter().enumerate() {
            if f(t) {
                return Some(i);
            }
        }
        None
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

    pub fn find_prev_where(
        &self,
        start_index: usize,
        filter: impl Fn(&Term) -> bool,
    ) -> Option<usize> {
        if self.terms.get(start_index).is_some() {
            self.terms
                .iter()
                .enumerate()
                .filter(|(i, t)| *i < start_index && filter(t))
                .rev()
                .map(|(i, _)| i)
                .next()
        } else {
            None
        }
    }

    pub fn find_next_where(
        &self,
        start_index: usize,
        filter: impl Fn(&Term) -> bool,
    ) -> Option<usize> {
        if self.terms.get(start_index).is_some() {
            self.terms
                .iter()
                .enumerate()
                .filter(|(i, t)| *i > start_index && filter(t))
                .map(|(i, _)| i)
                .next()
        } else {
            None
        }
    }

    pub fn find_last_where(&self, f: impl Fn(&Term) -> bool) -> Option<usize> {
        for (i, t) in self.terms.iter().enumerate().rev() {
            if f(t) {
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
        tags.iter().all(|t| self.tags.contains(*t))
    }

    pub fn any(&self, tags: &[Tag]) -> bool {
        tags.iter().any(|t| self.tags.contains(*t))
    }

    pub fn has_tag(&self, tag: Tag) -> bool {
        self.tags.contains(tag)
    }

    // Basic mutators

    pub fn add_tag(&mut self, t: Tag) {
        self.tags.insert(t);
    }

    pub fn add_tags(&mut self, tags: &[Tag]) {
        for t in tags {
            self.tags.insert(*t);
        }
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

    /// Applies the given operator.
    pub fn op(&mut self, code: Rule, operator: impl Fn(&mut Prakriya)) -> bool {
        operator(self);
        self.step(code);
        true
    }

    /// Applies the given operator to the given term.
    pub fn op_term(&mut self, code: Rule, index: usize, operator: impl Fn(&mut Term)) -> bool {
        if let Some(term) = self.get_mut(index) {
            operator(term);
            self.step(code);
            true
        } else {
            false
        }
    }

    /// Applies the given operator optionally.
    ///
    /// Returns: whether the operation was applied. This return value is required for certain
    /// complex conditions (e.g. 6.4.116 & 117; "if this rule was not applied, ...").
    pub fn op_optional(&mut self, code: Rule, operator: impl Fn(&mut Prakriya)) -> bool {
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
        if self.config.log_steps {
            let state = self.terms.iter().fold(String::new(), |a, b| {
                if a.is_empty() {
                    a + &b.text
                } else {
                    a + " + " + &b.text
                }
            });
            self.history.push(Step { rule, state })
        }
    }

    pub fn debug(&mut self, text: String) {
        self.history.push(Step { rule: "debug", state: text });
    }

    // Optional rules

    pub fn is_allowed(&mut self, r: Rule) -> bool {
        for option in &self.config.rule_choices {
            match option {
                RuleChoice::Accept(code) => {
                    if r == *code {
                        self.accept(r);
                        return true;
                    }
                }
                RuleChoice::Decline(code) => {
                    if r == *code {
                        return false;
                    }
                }
            }
        }

        // If not in options, allow this rule by default.
        self.accept(r);
        true
    }

    pub fn accept(&mut self, rule: Rule) {
        self.rule_decisions.push(RuleChoice::Accept(rule));
    }

    pub fn decline(&mut self, rule: Rule) {
        self.rule_decisions.push(RuleChoice::Decline(rule));
    }

    pub fn debug_print(&self) {
        for t in &self.terms {
            println!("- {t:?}");
        }
        println!("{:?}", self.tags);
    }

    // Final output

    pub fn text(&self) -> CompactString {
        let mut ret = CompactString::from("");
        for t in &self.terms {
            ret.push_str(&t.text);
        }
        ret
    }
}

/// Explores all optional derivations for some input.
///
/// Many of the rules in the Ashtadhyayi are optional, and by accepting or declining these optional
/// rules, we create different final results. `PrakriyaStack` manages the work required in finding
/// and exploring the various combinations of optional rules.
#[derive(Default)]
pub struct PrakriyaStack {
    /// Completed prakriyas.
    prakriyas: Vec<Prakriya>,
    /// Combinations of optional rules that we have yet to try.
    paths: Vec<Vec<RuleChoice>>,
}

impl PrakriyaStack {
    /// Creates an empty `PrakriyaStack`.
    pub fn new() -> Self {
        Self::default()
    }

    fn new_prakriya(rule_choices: Vec<RuleChoice>, log_steps: bool) -> Prakriya {
        Prakriya::with_config(Config {
            rule_choices,
            log_steps,
        })
    }

    /// Finds all variants of the given derivation function.
    ///
    /// `derive` should accept an empty `Prakriya` and mutate it in-place.
    pub fn find_all(&mut self, derive: impl Fn(&mut Prakriya), log_steps: bool) {
        let mut p_init = Self::new_prakriya(vec![], log_steps);
        derive(&mut p_init);
        self.add_prakriya(p_init, &[]);

        while let Some(path) = self.pop_path() {
            let mut p = Self::new_prakriya(path.clone(), log_steps);
            derive(&mut p);
            self.add_prakriya(p, &path);
        }
    }

    /// Adds a prakriya to the result set and adds new paths to the stack.
    ///
    /// We find new paths as follows. Suppose our initial prakriya followed the following path:
    ///
    /// > Accept(A), Accept(B), Accept(C)
    ///
    /// We then add one candidate path for each alternate choice we could have made:
    ///
    /// > Decline(A)
    /// > Accept(A), Decline(B)
    /// > Accept(A), Accept(B), Decline(C)
    ///
    /// Suppose we then try `Decline(A)` and make the following choices:
    ///
    /// > Decline(A), Accept(B), Accept(D)
    ///
    /// After this, adding an `Accept(A) path to the stack would be a mistake, as it would cause an
    /// infinite loop. Instead, we freeze our initial decision to use `Decline(A)` and add only the
    /// following paths:
    ///
    /// > Decline(A), Decline(B)
    /// > Decline(A), Accept(B), Decline(D)
    fn add_prakriya(&mut self, p: Prakriya, initial_choices: &[RuleChoice]) {
        let choices = p.rule_choices();
        let offset = initial_choices.len();
        for i in offset..choices.len() {
            let mut path = choices[..=i].to_vec();

            // Swap the last choice.
            let i = path.len() - 1;
            path[i] = match path[i] {
                RuleChoice::Accept(code) => RuleChoice::Decline(code),
                RuleChoice::Decline(code) => RuleChoice::Accept(code),
            };

            self.paths.push(path);
        }
        self.prakriyas.push(p);
    }

    /// Pops an unexplored choice path from the stack.
    fn pop_path(&mut self) -> Option<Vec<RuleChoice>> {
        self.paths.pop()
    }

    /// Returns all of the prakriyas this stack has found. This consumes the stack.
    pub fn prakriyas(self) -> Vec<Prakriya> {
        self.prakriyas
    }
}
