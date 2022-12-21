/*!
Common arguments for the crate's main functions.

Before we begin a prakriya, we must declare certain morphological information up-front, such as our
desired purusha and vacana, the dhatu we wish to use, and so on. To better document the API and to
help users avoid common typos, we model this information through the enums and structs in this module.

For extra flexibility, all of the enums here provides `as_str` and `from_str` methods. For details
on which strings are valid arguments in `from_str`, please read the source code directly.
*/
use crate::tag::Tag;
use compact_str::CompactString;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Indicates a failure to parse a string representation of some `semantics` enum.
#[derive(Debug, Clone)]
pub struct ArgumentError {
    /// The error message.
    msg: String,
}

impl ArgumentError {
    fn new(s: &str) -> Self {
        ArgumentError { msg: s.to_owned() }
    }
}

impl Error for ArgumentError {}

impl Display for ArgumentError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.msg)
    }
}

/// Defines an antargana.
///
/// The dhatus in the Dhatupatha are organized in ten large *gaṇa*s or classes. Within these larger
/// *gaṇa*s, certain *antargaṇa*s or subclasses have extra properties that affect the derivations
/// they produce. For example, dhatus in the *kuṭādi antargaṇa* generally do not allow *guṇa* vowel
/// changes.
///
/// Since most dhatus appear exactly once per *gaṇa*, this crate can usually infer whether a dhatu
/// is in a specific *antargaṇa*. However, some *gaṇa*s have dhatus that repeat, and these
/// repeating dhatus cause ambiguities for our code. (Examples: `juqa~` appears twice in
/// *tudādigaṇa*, once in *kuṭādi* and once outside of it.)
///
/// To avoid this ambiguity, we require that certain *antargaṇa*s are declared up-front.
///
/// (Can't we disambiguate by checking the dhatu's index within its gana? Unfortunately, no. There
/// is no canonical version of the Dhatupatha, and we cannot expect that a dhatu's index is
/// consistent across all of these versions. So we thought it better to avoid hard-coding indices
/// or requiring callers to follow our specific conventions.)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Antargana {
    /// Antargana of *tud* gana. Pratyayas that follow dhatus in kut-Adi will generally be marked
    /// Nit per 1.2.1. Required because of duplicates like `juqa~`.
    Kutadi,
    /// Antargana of *cur* gana ending with `kusma~`. A dhatu in this antargana is always
    /// ātmanepadī. Required because of duplicates like `daSi~`.
    Akusmiya,
}

/// The verb root to use for the derivation.
#[derive(Debug)]
pub struct Dhatu {
    /// The dhatu as stated in its aupadeshka form. `upadesha` should be an SLP1 string that
    /// includes any necessary svaras. For examples, see the `dhatu` column in the
    /// `data/dhatupatha.tsv` file included in this crate.
    pub upadesha: CompactString,
    /// The dhatu's gana. This should be a number between 1 and 10, inclusive.
    pub gana: u8,
    /// The antargana this Dhatu belongs to.
    pub antargana: Option<Antargana>,
}

impl Dhatu {
    /// Creates a new `Dhatu`.
    pub fn new(upadesha: impl AsRef<str>, gana: u8, antargana: Option<Antargana>) -> Self {
        Dhatu {
            upadesha: CompactString::from(upadesha.as_ref()),
            gana,
            antargana,
        }
    }

    /// Creates a convenient human-readable code for this dhatu. This code matches the format used
    /// on sites like ashtadhyayi.com.
    pub fn code(&self) -> String {
        format!("{:0>2}.{:?}", self.gana, self.antargana)
    }
}

/// The prayoga of some tinanta.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Prayoga {
    /// Usage coreferent with the agent, e.g. "The horse *goes* to the village."
    Kartari,
    /// Usage coreferent with the object, e.g. "The village *is gone to* by the horse."
    Karmani,
    /// Usage without a referent, e.g. "*There is motion* by the horse to the village."
    /// bhAve prayoga generally produces the same forms as karmani prayoga.
    Bhave,
}
impl Prayoga {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Kartari => Tag::Kartari,
            Self::Karmani => Tag::Karmani,
            Self::Bhave => Tag::Bhave,
        }
    }
    /// Returns a simple human-readable string that represents this enum's value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Kartari => "kartari",
            Self::Karmani => "karmani",
            Self::Bhave => "bhave",
        }
    }
}
impl FromStr for Prayoga {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "kartari" => Self::Kartari,
            "karmani" => Self::Karmani,
            "bhave" => Self::Bhave,
            &_ => return Err("Could not parse Prayoga"),
        };
        Ok(res)
    }
}

/// The person of some tinanta.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Purusha {
    /// The third person.
    Prathama,
    /// The second person.
    Madhyama,
    /// The first person.
    Uttama,
}
impl Purusha {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Prathama => Tag::Prathama,
            Self::Madhyama => Tag::Madhyama,
            Self::Uttama => Tag::Uttama,
        }
    }
    /// Returns a simple human-readable string that represents this enum's value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Prathama => "prathama",
            Self::Madhyama => "madhyama",
            Self::Uttama => "uttama",
        }
    }
}
impl FromStr for Purusha {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "prathama" => Self::Prathama,
            "madhyama" => Self::Madhyama,
            "uttama" => Self::Uttama,
            &_ => return Err("Could not parse Purusha"),
        };
        Ok(res)
    }
}

/// The number of some tinanta or subanta.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Vacana {
    /// The singular.
    Eka,
    /// The dual.
    Dvi,
    /// The plural.
    Bahu,
}
impl Vacana {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Eka => Tag::Ekavacana,
            Self::Dvi => Tag::Dvivacana,
            Self::Bahu => Tag::Bahuvacana,
        }
    }
    /// Returns a simple human-readable string that represents this enum's value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Eka => "eka",
            Self::Dvi => "dvi",
            Self::Bahu => "bahu",
        }
    }
}
impl FromStr for Vacana {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "eka" => Self::Eka,
            "dvi" => Self::Dvi,
            "bahu" => Self::Bahu,
            &_ => return Err("Could not parse Vacana"),
        };
        Ok(res)
    }
}

/// The gender of some subanta.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Linga {
    /// The masculine.
    Pum,
    /// The feminine.
    Stri,
    /// The neuter.
    Napumsaka,
}
impl Linga {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Pum => Tag::Pum,
            Self::Stri => Tag::Stri,
            Self::Napumsaka => Tag::Napumsaka,
        }
    }
    /// Returns a simple human-readable string that represents this enum's value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pum => "pum",
            Self::Stri => "stri",
            Self::Napumsaka => "napumsaka",
        }
    }
}
impl FromStr for Linga {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "pum" => Self::Pum,
            "stri" => Self::Stri,
            "napumsaka" => Self::Napumsaka,
            &_ => return Err("Could not parse Linga"),
        };
        Ok(res)
    }
}

/// The case ending of some subanta.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Vibhakti {
    /// The first vibhakti . Sometimes called the *nominative case*.
    Prathama,
    /// The second vibhakti. Sometimes called the *accusative case*.
    Dvitiya,
    /// The third vibhakti. Sometimes called the *instrumental case*.
    Trtiya,
    /// The fourth vibhakti. Sometimes called the *dative case*.
    Caturthi,
    /// The fifth vibhakti. Sometimes called the *ablative case*.
    Panchami,
    /// The sixth vibhakti. Sometimes called the *genitive case*.
    Sasthi,
    /// The seventh vibhakti. Sometimes called the *locative case*.
    Saptami,
    /// The first vibhakti used in the sense of *sambodhana*. Sometimes called the *vocative case*.
    ///
    /// *Sambodhana* is technically not a *vibhakti but rather an additional semantic condition
    /// that conditions the first vibhakti. But we felt that users would find it more convenient to
    /// have this condition available on `Vibhakti` directly rather than have to define the
    /// *sambodhana* condition separately.
    Sambodhana,
}
impl Vibhakti {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Prathama => Tag::V1,
            Self::Dvitiya => Tag::V2,
            Self::Trtiya => Tag::V3,
            Self::Caturthi => Tag::V4,
            Self::Panchami => Tag::V5,
            Self::Sasthi => Tag::V6,
            Self::Saptami => Tag::V7,
            Self::Sambodhana => Tag::V1,
        }
    }
    /// Returns a simple human-readable string that represents this enum's value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Prathama => "1",
            Self::Dvitiya => "2",
            Self::Trtiya => "3",
            Self::Caturthi => "4",
            Self::Panchami => "5",
            Self::Sasthi => "6",
            Self::Saptami => "7",
            Self::Sambodhana => "s",
        }
    }
}
impl FromStr for Vibhakti {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "1" => Self::Prathama,
            "2" => Self::Dvitiya,
            "3" => Self::Trtiya,
            "4" => Self::Caturthi,
            "5" => Self::Panchami,
            "6" => Self::Sasthi,
            "7" => Self::Saptami,
            "s" => Self::Sambodhana,
            &_ => return Err("Could not parse Vibhakti"),
        };
        Ok(res)
    }
}

/// The tense/mood of some tinanta.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Lakara {
    /// Describes action in the present tense. Ssometimes called the *present indicative*.
    Lat,
    /// Describes unwitnessed past action. Sometimes called the *perfect*.
    Lit,
    /// Describes future action after the current day. Sometimes called the *periphrastic future*.
    Lut,
    /// Describes general future action. Sometimes called the *simple future*.
    Lrt,
    /// The Vedic subjunctive. `vidyut-prakriya` currently has poor support for this lakara.
    Let,
    /// Describes commands. Sometimes called the *imperative*.
    Lot,
    /// Describes past action before the current day. Sometimes called the *imperfect*.
    Lan,
    /// Describes potential or hypothetical actions. Sometimes called the *optative*.
    VidhiLin,
    /// Describes wishes and prayers. Sometimes called the *benedictive*.
    AshirLin,
    /// Describes general past action. Sometimes called the *aorist*.
    Lun,
    /// Describes past counterfactuals ("would not have ..."). Sometimes called the *conditional*.
    Lrn,
}

impl Lakara {
    /// Returns a simple human-readable string that represents this enum's value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Lakara::Lat => "lat",
            Lakara::Lit => "lit",
            Lakara::Lut => "lut",
            Lakara::Lrt => "lrt",
            Lakara::Let => "let",
            Lakara::Lot => "lot",
            Lakara::Lan => "lan",
            Lakara::VidhiLin => "vidhi-lin",
            Lakara::AshirLin => "ashir-lin",
            Lakara::Lun => "lun",
            Lakara::Lrn => "lrn",
        }
    }

    /// Returns whether or not this lakara is Nit.
    pub(crate) fn is_nit(&self) -> bool {
        matches![
            self,
            Lakara::Lan | Lakara::AshirLin | Lakara::VidhiLin | Lakara::Lun | Lakara::Lrn
        ]
    }

    /// Returns whether or not this lakara will be termed sArvadhAtuka.
    pub(crate) fn is_sarvadhatuka(&self) -> bool {
        matches!(
            self,
            Lakara::Lat | Lakara::Lot | Lakara::Lan | Lakara::VidhiLin
        )
    }

    /// Returns whether or not this lakara will be termed ArdhadhAtuka.
    pub(crate) fn is_ardhadhatuka(&self) -> bool {
        !self.is_sarvadhatuka()
    }
}
impl FromStr for Lakara {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "lat" => Self::Lat,
            "lit" => Self::Lit,
            "lut" => Self::Lut,
            "lrt" => Self::Lrt,
            "let" => Self::Let,
            "lot" => Self::Lot,
            "lan" => Self::Lan,
            "vidhi-lin" => Self::VidhiLin,
            "ashir-lin" => Self::AshirLin,
            "lun" => Self::Lun,
            "lrn" => Self::Lrn,
            &_ => return Err("Could not parse La"),
        };
        Ok(res)
    }
}

/// The information required to derive a subanta in the grammar.
pub struct SubantaArgs {
    linga: Linga,
    vacana: Vacana,
    vibhakti: Vibhakti,
}

impl SubantaArgs {
    /// The linga to use in the derivation.
    pub fn linga(&self) -> Linga {
        self.linga
    }
    /// The vacana to use in the derivation.
    pub fn vacana(&self) -> Vacana {
        self.vacana
    }
    /// The vibhakti to use in the derivation.
    pub fn vibhakti(&self) -> Vibhakti {
        self.vibhakti
    }

    /// Returns a new builder for this struct.
    pub fn builder() -> SubantaArgsBuilder {
        SubantaArgsBuilder::default()
    }
}

/// Convenience struct for building a `SubantaArgs` object.
#[derive(Default)]
pub struct SubantaArgsBuilder {
    linga: Option<Linga>,
    vacana: Option<Vacana>,
    vibhakti: Option<Vibhakti>,
}

impl SubantaArgsBuilder {
    /// Sets the linga to use in the derivation.
    pub fn linga(&mut self, val: Linga) -> &mut Self {
        self.linga = Some(val);
        self
    }
    /// Sets the vacana to use in the derivation.
    pub fn vacana(&mut self, val: Vacana) -> &mut Self {
        self.vacana = Some(val);
        self
    }
    /// Sets the vibhakti to use in the derivation.
    pub fn vibhakti(&mut self, val: Vibhakti) -> &mut Self {
        self.vibhakti = Some(val);
        self
    }

    /// Converts the arguments in this builder into a `SubantaArgs` struct.
    ///
    /// `build()` will fail if any args are missing.
    pub fn build(&self) -> Result<SubantaArgs, ArgumentError> {
        Ok(SubantaArgs {
            linga: match self.linga {
                Some(x) => x,
                _ => return Err(ArgumentError::new("foo")),
            },
            vacana: match self.vacana {
                Some(x) => x,
                _ => return Err(ArgumentError::new("foo")),
            },
            vibhakti: match self.vibhakti {
                Some(x) => x,
                _ => return Err(ArgumentError::new("foo")),
            },
        })
    }
}

/// The information required to derive a tinanta in the grammar.
pub struct TinantaArgs {
    prayoga: Prayoga,
    purusha: Purusha,
    lakara: Lakara,
    vacana: Vacana,
}

impl TinantaArgs {
    /// The linga to use in the derivation.
    pub fn prayoga(&self) -> Prayoga {
        self.prayoga
    }
    /// The purusha to use in the derivation.
    pub fn purusha(&self) -> Purusha {
        self.purusha
    }
    /// The lakara to use in the derivation.
    pub fn lakara(&self) -> Lakara {
        self.lakara
    }
    /// The vacana to use in the derivation.
    pub fn vacana(&self) -> Vacana {
        self.vacana
    }

    /// Returns a new builder for this struct.
    pub fn builder() -> TinantaArgsBuilder {
        TinantaArgsBuilder::default()
    }
}

/// Convenience struct for building a `TinantaArgs` object.
#[derive(Default)]
pub struct TinantaArgsBuilder {
    prayoga: Option<Prayoga>,
    purusha: Option<Purusha>,
    lakara: Option<Lakara>,
    vacana: Option<Vacana>,
}

impl TinantaArgsBuilder {
    /// Sets the prayoga to use in the derivation.
    pub fn prayoga(&mut self, val: Prayoga) -> &mut Self {
        self.prayoga = Some(val);
        self
    }
    /// Sets the purusha to use in the derivation.
    pub fn purusha(&mut self, val: Purusha) -> &mut Self {
        self.purusha = Some(val);
        self
    }
    /// Sets the lakara to use in the derivation.
    pub fn lakara(&mut self, val: Lakara) -> &mut Self {
        self.lakara = Some(val);
        self
    }
    /// Sets the vacana to use in the derivation.
    pub fn vacana(&mut self, val: Vacana) -> &mut Self {
        self.vacana = Some(val);
        self
    }

    /// Converts the arguments in this builder into a `TinantaArgs` struct.
    ///
    /// `build()` will fail if any args are missing.
    pub fn build(&self) -> Result<TinantaArgs, ArgumentError> {
        Ok(TinantaArgs {
            prayoga: match self.prayoga {
                Some(x) => x,
                _ => return Err(ArgumentError::new("foo")),
            },
            purusha: match self.purusha {
                Some(x) => x,
                _ => return Err(ArgumentError::new("foo")),
            },
            lakara: match self.lakara {
                Some(x) => x,
                _ => return Err(ArgumentError::new("foo")),
            },
            vacana: match self.vacana {
                Some(x) => x,
                _ => return Err(ArgumentError::new("foo")),
            },
        })
    }
}
