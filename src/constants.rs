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
    VIKARANA,

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
    Nit,
    cit,
    Yit,
    wit,
    Rit,
    nit,
    pit,
    mit,
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
            "N" => Tag::Nit,
            "c" => Tag::cit,
            "Y" => Tag::Yit,
            "w" => Tag::wit,
            "R" => Tag::Rit,
            "n" => Tag::nit,
            "p" => Tag::pit,
            "m" => Tag::mit,
            "S" => Tag::Sit,
            "z" => Tag::zit,
            _ => panic!("Fix this later"),
        };
        Ok(res)
    }
}
