use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

type Sound = String;

lazy_static! {
    static ref SUTRAS: Vec<Sutra> = create_shiva_sutras();
    static ref SOUND_PROPS: HashMap<Sound, Uccarana> = create_sound_props();
}

fn create_shiva_sutras() -> Vec<Sutra> {
    vec![
        Sutra::new(vec!["a", "i", "u"], "R"),
        Sutra::new(vec!["f", "x"], "k"),
        Sutra::new(vec!["e", "o"], "N"),
        Sutra::new(vec!["E", "O"], "c"),
        Sutra::new(vec!["ha", "ya", "va", "ra"], "w"),
        Sutra::new(vec!["la"], "R"),
        Sutra::new(vec!["Ya", "ma", "Na", "Ra", "na"], "m"),
        Sutra::new(vec!["Ja", "Ba"], "Y"),
        Sutra::new(vec!["Ga", "Qa", "Da"], "z"),
        Sutra::new(vec!["ja", "ba", "ga", "qa", "da"], "S"),
        Sutra::new(vec!["Ka", "Pa", "Ca", "Wa", "Ta", "ca", "wa", "ta"], "v"),
        Sutra::new(vec!["ka", "pa"], "y"),
        Sutra::new(vec!["Sa", "za", "sa"], "r"),
        Sutra::new(vec!["ha"], "l"),
    ]
}

fn create_sound_props() -> HashMap<Sound, Uccarana> {
    fn flatten_multi<T: Copy>(data: Vec<(SoundSet, T)>) -> HashMap<Sound, Vec<T>> {
        let mut mapping = HashMap::new();
        for (ks, v) in data {
            for k in &ks.vec {
                mapping.entry(k.clone()).or_insert_with(Vec::new).push(v);
            }
        }
        mapping
    }
    fn flatten<T: Copy>(data: Vec<(SoundSet, T)>) -> HashMap<Sound, T> {
        let mut mapping = HashMap::new();
        for (ks, v) in data {
            for k in &ks.vec {
                mapping.insert(k.clone(), v);
            }
        }
        mapping
    }

    let mut sthana = flatten_multi(vec![
        (s("a ku~ h H"), Sthana::Kantha),
        (s("i cu~ y S"), Sthana::Talu),
        (s("f wu~ r z"), Sthana::Murdha),
        (s("x tu~ l s"), Sthana::Danta),
        (s("u pu~"), Sthana::Oshtha),
        (s("e E"), Sthana::KanthaTalu),
        (s("o O"), Sthana::KanthaOshtha),
        (s("v"), Sthana::DantaOshtha),
    ]);
    for k in s("Yam M").vec {
        sthana
            .entry(k)
            .or_insert_with(Vec::new)
            .push(Sthana::Nasika);
    }

    let ghosha = flatten(vec![
        (s("ac haS M"), Ghosha::Ghoshavat),
        (s("Kar H"), Ghosha::Aghosha),
    ]);
    let prana = flatten(vec![
        (s("ac yam jaS car M"), Prana::Alpaprana),
        (s("K G C J W Q T D P B h"), Prana::Mahaprana),
    ]);
    let prayatna = flatten(vec![
        (s("yaR Sar"), Prayatna::Ishat),
        (s("ac h"), Prayatna::Vivrta),
        (s("Yay"), Prayatna::Sprshta),
    ]);

    let mut res = HashMap::new();
    for k in s("al H M").vec {
        let sthana = match sthana.get(&k) {
            Some(s) => s.clone(),
            None => Vec::new(),
        };

        res.insert(
            k.clone(),
            Uccarana {
                sthana,
                ghosha: *ghosha.get(&k).unwrap_or(&Ghosha::Aghosha),
                prana: *prana.get(&k).unwrap_or(&Prana::Alpaprana),
                prayatna: *prayatna.get(&k).unwrap_or(&Prayatna::Vivrta),
            },
        );
    }

    res
}

struct Uccarana {
    sthana: Vec<Sthana>,
    ghosha: Ghosha,
    prana: Prana,
    prayatna: Prayatna,
}

impl Uccarana {
    fn distance(&self, other: &Uccarana) -> i32 {
        let mut dist = 0;
        if self.ghosha != other.ghosha {
            dist += 1;
        };
        if self.prana != other.prana {
            dist += 1;
        }
        if self.prayatna != other.prayatna {
            dist += 1;
        }

        let mut sthana_dist = (self.sthana.len() + other.sthana.len()) as i32;
        for s in &self.sthana {
            if other.sthana.contains(s) {
                sthana_dist -= 2;
            }
        }

        dist + sthana_dist
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Sthana {
    Kantha,
    Talu,
    Murdha,
    Danta,
    Oshtha,
    Nasika,
    KanthaTalu,
    KanthaOshtha,
    DantaOshtha,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Ghosha {
    Ghoshavat,
    Aghosha,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Prana {
    Mahaprana,
    Alpaprana,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Prayatna {
    Vivrta,
    Ishat,
    Sprshta,
}

pub struct SoundSet {
    vec: Vec<String>,
    set: HashSet<String>,
}

impl SoundSet {
    pub fn new(sounds: Vec<Sound>) -> Self {
        let set = sounds.iter().cloned().collect();
        SoundSet { set, vec: sounds }
    }

    pub fn contains(&self, s: &str) -> bool {
        self.set.contains(s)
    }
    pub fn contains_char(&self, c: char) -> bool {
        self.set.contains(&c.to_string())
    }
    pub fn contains_opt(&self, o: Option<char>) -> bool {
        if let Some(c) = o {
            self.contains_char(c)
        } else {
            false
        }
    }

    pub fn items(&self) -> &Vec<Sound> {
        &self.vec
    }
}

struct Sutra {
    sounds: Vec<String>,
    it: String,
}

impl Sutra {
    fn new(sounds: Vec<&str>, it: &str) -> Self {
        Sutra {
            sounds: sounds.iter().map(|x| x.to_string()).collect(),
            it: it.to_string(),
        }
    }
}

fn pratyahara(s: &str) -> SoundSet {
    let n = s.len();

    let use_second_n = s.ends_with("R2");
    let (first, it) = if use_second_n {
        (&s[..n - 2], &s[n - 2..n - 1])
    } else {
        (&s[..n - 1], &s[n - 1..])
    };

    let mut started = false;
    let mut saw_first_n = false;
    let mut res = vec![];

    for sutra in SUTRAS.iter() {
        for sound in &sutra.sounds {
            if first == sound {
                started = true;
            }
            if started {
                let letter = &sound[0..=0];
                res.push(letter.to_string());

                // Add long vowels, which are not explictly included in the
                // Shiva Sutras.
                match letter {
                    "a" | "i" | "u" | "f" | "x" => {
                        res.push(letter.to_uppercase());
                    }
                    _ => (),
                }
            }
        }

        if started && it == sutra.it {
            if use_second_n && !saw_first_n {
                saw_first_n = true;
            } else {
                break;
            }
        }
    }

    assert!(!res.is_empty(), "Could not parse pratyahara `{s}`");
    SoundSet::new(res)
}

pub fn savarna(c: char) -> SoundSet {
    let sounds = match c {
        'a' | 'A' => "aA",
        'i' | 'I' => "iI",
        'u' | 'U' => "uU",
        'f' | 'F' | 'x' | 'X' => "fFxX",
        'k' | 'K' | 'g' | 'G' | 'N' => "kKgGN",
        'c' | 'C' | 'j' | 'J' | 'Y' => "cCjJY",
        'w' | 'W' | 'q' | 'Q' | 'R' => "wWqQR",
        't' | 'T' | 'd' | 'D' | 'n' => "tTdDn",
        'p' | 'P' | 'b' | 'B' | 'm' => "pPbBm",
        _ => return SoundSet::new(vec![c.to_string()]),
    };
    SoundSet::new(sounds.chars().map(|x| x.to_string()).collect())
}

pub fn s(terms: &str) -> SoundSet {
    let mut ret = vec![];
    let ak = ["a", "A", "i", "I", "u", "U", "f", "F", "x", "X"];

    for term in terms.split_whitespace() {
        if term.ends_with("u~") || ak.contains(&term) {
            let first = term.chars().next().unwrap();
            ret.extend(savarna(first).vec);
        } else if term.len() == 1 {
            ret.push(term.to_string());
        } else {
            ret.extend(pratyahara(term).vec);
        }
    }

    SoundSet::new(ret)
}

pub fn map_sounds(xs: &str, ys: &str) -> HashMap<Sound, Sound> {
    let xs = s(xs);
    let ys = s(ys);

    let mut mapping = HashMap::new();
    for x in xs.vec {
        let x_props = SOUND_PROPS.get(&x).unwrap();

        // The best sound has the minimal distance.
        let best_y = ys
            .items()
            .iter()
            .min_by_key(|y| SOUND_PROPS.get(*y).unwrap().distance(x_props))
            .unwrap();

        let d = x_props.distance(SOUND_PROPS.get(best_y).unwrap());
        println!("The closest sound to {x} is {best_y} with distance {d}.");
        mapping.insert(x, best_y.clone());
    }

    mapping
}

fn to_guna(s: &str) -> &'static str {
    match s {
        "i" | "I" => "e",
        "u" | "U" => "o",
        "f" | "F" => "ar",
        "x" | "X" => "al",
        &_ => panic!("Invalid guna sound {s}"),
    }
}

fn to_vrddhi(s: &str) -> &'static str {
    match s {
        "a" | "A" => "A",
        "i" | "I" => "E",
        "u" | "U" => "O",
        "f" | "F" => "Ar",
        "x" | "X" => "Al",
        "e" | "E" => "E",
        "o" | "O" => "O",
        &_ => panic!("Invalid vrddhi sound {s}"),
    }
}

// 1.1.48 UkAlojjhrasvadIrghaplutaH
fn to_hrasva(s: &str) -> &'static str {
    match s {
        "a" | "A" => "a",
        "i" | "I" => "i",
        "u" | "U" => "u",
        "f" | "F" => "f",
        "x" | "X" => "x",
        "e" | "E" => "i",
        "o" | "O" => "u",
        &_ => panic!("Invalid hrasva sound {s}"),
    }
}

// 1.1.48 UkAlojjhrasvadIrghaplutaH
fn to_dirgha(s: &str) -> &'static str {
    match s {
        "a" | "A" => "A",
        "i" | "I" => "U",
        "u" | "U" => "U",
        "f" | "F" => "F",
        "x" | "X" => "X",
        "e" => "e",
        "E" => "E",
        "o" => "o",
        "O" => "O",
        &_ => panic!("Invalid dirgha sound {s}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s() {
        let tests = vec![
            ("ac", "aAiIuUfFxXeEoO"),
            ("iR", "iIuU"),
            ("iR2", "iIuUfFxXeEoOhyvrl"),
            ("yaR", "yrvl"),
            ("hal", "kKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzsh"),
            ("Yam", "YmNRn"),
            ("Sar", "Szs"),
            ("a", "aA"),
            ("e", "e"),
            ("ku~", "kKgGN"),
            ("cu~", "cCjJY"),
            ("i cu~", "iIcCjJY"),
            ("a ku~ h H", "aAkKgGNhH"),
        ];
        for (input, expected) in tests {
            let expected = expected.chars().map(|x| x.to_string()).collect();
            let actual = s(input).set;
            assert_eq!(actual, expected, "input: `{input}`");
        }
    }

    #[test]
    fn test_map_sounds_jhal_jhash() {
        let actual = map_sounds("Jal", "jaS");
        let expected: HashMap<String, String> = vec![
            ("J", "j"),
            ("B", "b"),
            ("G", "g"),
            ("Q", "q"),
            ("D", "d"),
            ("j", "j"),
            ("b", "b"),
            ("g", "g"),
            ("q", "q"),
            ("d", "d"),
            ("K", "g"),
            ("P", "b"),
            ("C", "j"),
            ("W", "q"),
            ("T", "d"),
            ("c", "j"),
            ("w", "q"),
            ("t", "d"),
            ("k", "g"),
            ("p", "b"),
            ("S", "j"),
            ("z", "q"),
            ("s", "d"),
            ("h", "g"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_map_sounds_kuh_cu() {
        let actual = map_sounds("ku~ h", "cu~");
        let expected: HashMap<String, String> = vec![
            ("k", "c"),
            ("K", "C"),
            ("g", "j"),
            ("G", "J"),
            ("N", "Y"),
            ("h", "J"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

        assert_eq!(expected, actual);
    }
}
