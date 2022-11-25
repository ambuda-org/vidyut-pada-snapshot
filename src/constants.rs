use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

/// Indicates a failure to parse a string representation of some `semantics` enum.
#[derive(Debug, Clone)]
pub struct PrakriyaError {
    /// The error message.
    msg: String,
}

impl PrakriyaError {
    fn new(s: &str) -> Self {
        PrakriyaError { msg: s.to_owned() }
    }
}

impl Error for PrakriyaError {}

impl Display for PrakriyaError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Prayoga {
    Kartari,
    Karmani,
    Bhave,
}
impl Prayoga {
    pub fn as_tag(&self) -> Tag {
        match self {
            Self::Kartari => Tag::Kartari,
            Self::Karmani => Tag::Karmani,
            Self::Bhave => Tag::Bhave,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Purusha {
    Prathama,
    Madhyama,
    Uttama,
}
impl Purusha {
    pub fn as_tag(&self) -> Tag {
        match self {
            Self::Prathama => Tag::Prathama,
            Self::Madhyama => Tag::Madhyama,
            Self::Uttama => Tag::Uttama,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Vacana {
    Eka,
    Dvi,
    Bahu,
}
impl Vacana {
    pub fn as_tag(&self) -> Tag {
        match self {
            Self::Eka => Tag::Ekavacana,
            Self::Dvi => Tag::Dvivacana,
            Self::Bahu => Tag::Bahuvacana,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum La {
    Lat,
    Lit,
    Lut,
    Lrt,
    Let,
    Lot,
    Lan,
    VidhiLin,
    AshirLin,
    Lun,
    Lrn,
}

impl La {
    pub fn is_nit(&self) -> bool {
        matches![
            self,
            La::Lan | La::AshirLin | La::VidhiLin | La::Lun | La::Lrn
        ]
    }
    pub fn is_sarvadhatuka(&self) -> bool {
        matches!(self, La::Lat | La::Lot | La::Lan | La::VidhiLin)
    }
    pub fn is_ardhadhatuka(&self) -> bool {
        !self.is_sarvadhatuka()
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Tag {
    // Morpheme types
    Upasarga,
    Dhatu,
    Ghu,
    Agama,
    Pratyaya,
    Pratipadika,
    Vibhakti,
    Sarvanama,
    Sarvanamasthana,
    Tin,
    Nistha,
    Krt,
    Krtya,
    Sup,
    Taddhita,
    Vikarana,

    // It
    adit,
    Adit,
    idit,
    Idit,
    udit,
    Udit,
    fdit,
    xdit,
    edit,
    odit,
    kit,
    Kit,
    Git,
    Nit,
    cit,
    Cit,
    jit,
    Jit,
    Yit,
    wit,
    qit,
    Qit,
    Rit,
    nit,
    pit,
    Pit,
    mit,
    lit,
    Sit,
    zit,

    irit,
    YIt,
    wvit,
    qvit,

    // Lopa
    Luk,
    Slu,
    Lup,

    // Accent
    Anudatta,
    Svarita,
    anudattet,
    svaritet,

    // Pada
    Parasmaipada,
    Atmanepada,

    // Artha (semantic conditions)
    Ashih,
    Sanartha,
    Yanartha,

    // Dialect conditions
    Chandasi,

    // Prayoga
    Kartari,
    Bhave,
    Karmani,

    // Purusha
    Prathama,
    Madhyama,
    Uttama,

    // Vacana
    Ekavacana,
    Dvivacana,
    Bahuvacana,

    // Vibhakti (subanta)
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,

    // Linga (subanta)
    Pum,
    Stri,
    Napumsaka,

    // Stem types
    Nadi,
    Ghi,

    // Vibhakti conditions
    Sambodhana,
    Amantrita,
    Sambuddhi,

    // Dvitva
    Abhyasa,
    Abhyasta,

    // Dhatuka
    Ardhadhatuka,
    Sarvadhatuka,

    // Other flags
    //
    // Certain conditions cross prakaranas in a way that is difficult to track.
    // Since these conditions are limited, we just keep track of them with
    // these flags.

    // Flags on the `Term`:
    FlagGunaApavada,
    FlagGuna,

    // Flags on the `Prakriya`.
    FlagAdeshadi,
    FlagNoArdhadhatuka,
    FlagAnitKsa,
    FlagSetSic,
    FlagAtAgama,
    FlagAtLopa,

    Sat,
    Snam,
}

impl Tag {
    pub fn parse_it(it: &str) -> Result<Tag, Box<dyn Error>> {
        let res = match it {
            "a" => Tag::adit,
            "A" => Tag::Adit,
            "i" => Tag::idit,
            "I" => Tag::Idit,
            "u" => Tag::udit,
            "U" => Tag::Udit,
            "f" => Tag::fdit,
            "x" => Tag::xdit,
            "e" => Tag::edit,
            "o" => Tag::odit,
            "k" => Tag::kit,
            "K" => Tag::Kit,
            "G" => Tag::Git,
            "N" => Tag::Nit,
            "c" => Tag::cit,
            "C" => Tag::Cit,
            "j" => Tag::jit,
            "J" => Tag::Jit,
            "Y" => Tag::Yit,
            "w" => Tag::wit,
            "q" => Tag::qit,
            "Q" => Tag::Qit,
            "R" => Tag::Rit,
            "n" => Tag::nit,
            "p" => Tag::pit,
            "P" => Tag::Pit,
            "m" => Tag::mit,
            "l" => Tag::lit,
            "S" => Tag::Sit,
            "z" => Tag::zit,
            _ => panic!("Unknown it letter {it}"),
        };
        Ok(res)
    }
}
