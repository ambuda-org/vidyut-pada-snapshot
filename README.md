vidyut-prakriya
===============

`vidyut-prakriya` generates Sanskrit words with their prakriyās (derivations)
according to the rules of Paninian grammar.

This crate is under active development as part of the [Ambuda][ambuda] project.
If you enjoy our work and wish to contribute, we encourage you to [join our
Discord server][discord].

- [Overview][#overview]
- [Usage][#usage]
- [Technical design][#technical-design]
- [Roadmap][#roadmap]


[ambuda]: https://ambuda.org
[discord]: https://discord.gg/7rGdTyWY7Z


Overview
--------

`vidyut-prakriya` has three distinguishing qualities:

1. *Fidelity*. We follow the rules of Paninian grammar as closely as possible.
   Each word we return can optionally include a prakriyā that lists each rule
   that was used as well as its result.

2. *Speed*. On my laptop (a 2.4GHz 8-core CPU with 64 GB of DDR4 RAM), this
   crate generates almost 100,000 words per second. All else equal, a fast
   program is easier to run and test, which means that we can produce a larger
   word list at a higher standard of quality.

3. *Portability*. This crate compiles to native code and can be bound to most
   other progamming languages with some effort. In particular, this crate can
   be combined to WebAssembly, which means that it can run in a modern web
   browser.

`vidyut-prakriya` currently has strong support for basic verbs. But long-term,
we want `vidyut-prakriya` to generate every pada allowed by the rules of
Paninian grammar. For details, see our [roadmap][#roadmap].


Usage
-----

First, install Rust on your computer. You can find installation instructions
[here][install-rust].

Second, download `vidyut-prakriya` to your computer:

```
$ git clone git@github.com:ambuda-org/vidyut-prakriya-snapshot.git
$ cd vidyut-prakriya-snapshot
```

To generate all basic tinantas in kartari prayoga, run:

```
$ make create_tinantas
```

The first run of `make create_tinantas` will be slow since your machine must
first compile `vidyut-prakriya`. After this initial compilation step, `make
create_tinantas` will typically compile and complete within a few seconds.

To generates prakriyas programmatically, you can use the starter code below:

```rust
use vidyut_prakriya::ashtadhyayi;
use vidyut_prakriya::arguments::{La, Prayoga, Purusha, Vacana};

let prakriyas = ashtadhyayi::derive_tinantas(
    "BU",
    "01.0001",
    La::Lat,
    Prayoga::Kartari,
    Purusha::Prathama,
    Vacana::Eka,
    true,
);

for p in prakriyas {
    println!("{}", p.text());
}
```


[install-rust]: https://www.rust-lang.org/tools/install
[sv]: https://github.com/drdhaval2785/SanskritVerb



Technical design
----------------

`vidyut-prakriya` follows the form and spirit of the Ashtadhyayi as closely as
possible. At the same time, we make certain concessions to pragmatism so that
we can build a clear and maintainable program. For example, instead of
selecting a rule according to principles like `utsarga-apavAda`, we instead
manually reorder rules so that we can run a simple imperative program.

Our main data structure is a `Term`, which is a string with associated
metadata that we use during the derivation. `Term` has an expressive API that
lets us express Paninian rules readably and concisely.

We manage the overall derivation with a `Prakriya`, which a `Vec<Term>` along
with associated metadata and a log of which steps were applied in the
derivation.

In general, rules have a basic structure:

```rust
if meets_condition(p) {
    p.op("1.2.3", some_operator)
}
```

where:
- `p` is a `Prakriya`. We abbreviate this to `p`.
- `op` applies the `some_operator` function to `p` and records that rule
  `"1.2.3"` was applied.
- `some_operator` is a function that alters the prakriya in some way.


Roadmap
-------

*(This section uses Paninian terms that might be difficult for a general reader to
understand.)*

For tinantas, we aim to produce all valid combinations of (`upasarga`, `dhatu`,
`sanadi`, `prayoga`, `purusha`, `vacana`, `lakara`, `pada`), where:

- `upasarga` is a group of zero or more upasargas, focusing on common
  combinations.
- `dhatu` is a mUla-dhAtu (basic verb root) from the Dhatupatha.
- `sanadi` is an optional *san*, *nic*, *yan*, or *yan-luk* pratyaya.
- `purusha` is one of {prathama, madhyama, uttama}
- `vacana` is one of {ekavacana, dvivacana, bahuvacana}
- `lakara` is any lakara, excluding `let`.

For krt-subantas, we aim to produce all valid combinations of (`upasarga`, `dhatu`,
`sanadi`, `krt`, `linga`, `vibhakti`, `vacana`), where:

- `upasarga`, `dhatu`, and `sanadi` are as above.
- `krt` is any `krt`-pratyaya introduced in the Ashtadhyayi, including the
  uNAdi-sUtras but excluding chAndasa usage. 
- `linga` is one of {pum, stri, napumsaka}
- `vibhakti` is one of the seven vibhaktis or sambodhana.
- `vacana` is as above.

For all other subantas, we aim to produce all valid combinations of
(`pratipadika`, `linga`, `vibhakti`, `vacana`), where:

- `pratipadika` is a stem listed in a standard dictionary.
- `linga`, `vibhakti`, and `vacana` are as above.
