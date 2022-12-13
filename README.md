vidyut-pada
===========

`vidyut-pada` generates Sanskrit words with their prakriyās (derivations)
according to the rules of Paninian grammar.

Currently, `vidyut-pada` generates basic verbs in `kartari-prayoga`. For our
future plans, see the *Roadmap* section below.

`vidyut-pada` is under active development as part of the [Ambuda][ambuda]
project. If you enjoy our work and wish to contribute, we encourage you to
[join our Discord server][discord].

[ambuda]: https://ambuda.org
[discord]: https://discord.gg/7rGdTyWY7Z


Overview
--------

Many Sanskrit programs need access to a large and reliable **word list**. But
creating such a word list is challenging: a single Sanskrit word base can
produce thousands of different words, and all of these words must adhere to
Sanskrit's complex morphological rules. 

`vidyut-pada` is a Sanskrit word generator with three main features:

1. *Fidelity*. `vidyut-pada` follows the rules of Paninian grammar as closely
   as possible. Each word it returns includes an optional prakriyā (derivation)
   that lists each rule that was used as well as its result.

2. *Speed*. When run on multiple threads, `vidyut-pada` generates hundreds of
   thousands of words per second. A fast program, all else equal, is easier to
   test and run, which means that we can produce a larger word list at a higher
   standard of quality.

3. *Portability*. `vidyut-pada` compiles to native code, and it is easy to bind
   to other languages. In particular, `vidyut-pada` can be combined to
   WebAssembly, which means that it can run in a modern web browser.


Setup
-----

First, install Rust on your computer. You can find installation instructions
[here][install-rust].

Second, download `vidyut-pada` to your computer:

```
$ git clone git@github.com:ambuda-org/vidyut-pada.git
$ cd vidyut-pada
```

Finally, generate a list of tinantas:

```
$ make tinantas
```

The first run of `make tinantas` will be slow since it takes some time to
compile `vidyut-pada`. After initial compilation, `make tinantas` will
typically compile and complete within 10 seconds.


[install-rust]: http://
[sv]: https://github.com/drdhaval2785/SanskritVerb



Technical design
----------------

`vidyut-pada` follows the Ashtadhyayi as closely as possible. At the same time,
we make certain concessions to pragmatism so that we can build a clear and
maintainable program. For example, instead of selecting a rule according to
principles like `utsarga-apavAda`, we instead manually reorder rules so that we
can run a simple imperative program.

~

We represent each step of the derivation as a list of `Term`s, where a `Term`
is a string that contains metadata like whether the term is a dhatu, a
pratyaya, etc. By building a rich API on top of `Term`, we give ourselves a
terse language for representing Paninian rules.

In general, rules have a basic structure:

```rust
    if condition(p) {
        p.op("1.2.3", some_operator)
    }
```

where:
- `condition` is a boolean
- `p` is the prakriya. We abbreviate this to `p`.
- `op` applies the `some_operator` to `p` and records that rule `"1.2.3"` was applied.
- `some_operator` is a function that alters the prakriya in some way.


Roadmap
-------

*(This section uses Paninian terms that might be hard for a general reader to
understand.)*

Since the Sanskrit word list is infinite, `vidyut-pada` cannot possibly produce
every Sanskrit word. Instead, `vidyut-pada` focuses on an interesting subset
that should be useful for most applications.

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

