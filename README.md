# Advent Of Code
In this repo I'll update my solutions to the yearly challenge of [**Advent Of Code**](https://adventofcode.com/) by **Eric Wastl**.


# Languages
## Python
I chose Python because it's flexibility allows me to try different aproaches quickly. It's is my go-to language when I've to prototype and iterate.

## Crystal
[Crystal](https://crystal-lang.org/) is almost as easy to write as Python and runs almost as fast as C. It may be a young language but it's a lot of fun to write!

## Julia
[Julia](https://julialang.org/) is a really powerful language. It may not have Python's scripting features but in my opinion it's a much better option for anything computationally intensive.

## Rust
[Rust](https://doc.rust-lang.org/stable/book/title-page.html) is amazing. I refuse to elaborate.


# Project Structure
The solutions will be separated by year and day. The input will be placed next to the sources and binaries will be placed in a `build` folder. Both the input and the binaries will be git-ignored.

Example:
- 2020
  - day-1
    - build
      * rust
      * crystal
    * `python.py`
    * `rust.rs`
    * `crystal.cr`
    * `input`
  - day-2
    - build
      * rust
    * `python.py`
    * `rust.rs`
    * `julia.jl`
    * `input`
- 2021
  - day-1


# Benchmarks

I like to benchmark my solutions with [sharkdp's Hyperfine](https://github.com/sharkdp/hyperfine). Here's an example of what it looks like:

```console
hyperfine --warmup 10 -m 1500 'python3.9 python.py' './build/crystal' './build/rust' --export-markdown build/day-2-report.md
```

| Command               |  Mean [ms] | Min [ms] | Max [ms] |     Relative |
| :-------------------- | ---------: | -------: | -------: | -----------: |
| `python3.9 python.py` | 16.3 ± 3.8 |      9.5 |     22.0 | 19.37 ± 9.18 |
| `./build/crystal`     |  1.2 ± 0.1 |      0.9 |      2.1 |  1.38 ± 0.58 |
| `./build/rust`        |  0.8 ± 0.3 |      0.3 |      4.1 |         1.00 |
