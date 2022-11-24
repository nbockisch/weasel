# Weasel
This is an implementation of the [weasel genetic algorithm](https://en.wikipedia.org/wiki/Weasel_program) I wrote for fun. It
allows for customization of accepted character sets, number of candidates
generated per generation, and the mutation rate. It prints the best candidate
per generation like so:

```
// Here a mutation rate is provided of 10% for each candidate
$ ./weasel -p "Hello!" -m 10
Start: pqeIJu
Gen 0: pqeIJu
Gen 1: peeIJu
-- snip --
Gen 10: Helle!
Gen 11: Hello!
```

# Building
This project was compiled using `rustc` and `cargo` version 1.64.0, though
earlier versions will likely work as well. Check [the Rust website](https://www.rust-lang.org/) for more on
those tools

```
git clone git@github.com:nbockisch/weasel.git
cd weasel
cargo build --release // The binary will be in target/release
```

# Usage
```
An implementation of the Weasel genetic algorithm

Usage: weasel [OPTIONS] --phrase <PHRASE>

Options:
  -p, --phrase <PHRASE>
          The phrase to run the algorithm on
  -c, --char-set <CHAR_SET>
          The approved character set [default: "ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz!?."]
  -i, --iterations <ITERATIONS>
          The number of variations to produce per generation, >= 1 [default: 100]
  -m, --mutation-rate <MUTATION_RATE>
          The mutation rate for each string, from 1-100 [default: 5]
  -h, --help
          Print help information
  -V, --version
          Print version information
```
