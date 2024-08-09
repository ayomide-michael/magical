# Magical Programming Language


Welcome to the *Magical* programming language.
This README outlines the development plan and steps for the initial phase of the *Magical* programming language.

## Project Overview:
The goal of this project is to design and implement the *Magical* programming language, with the initial phase focusing on the parser component, which is the foundation for interpreting and compiling code written in *Magical*.

## Phase 1: Parser Development

### Week 1: Building the basic Parser

#### Objectives:
1. Implement a basic `Parser Combinators` from scratch.
2. Test the `Parser` to ensure it works as expected.

#### Tasks:
- Set up the project structure.
- Implement basic `Parser` with:
    - `Parser<F, T>`: concrete parser type.
    - `Context`: to manage the parser state.
    - `Error`: an enum type to represent parse error.
    - `Node<T>`: parse tree structure returned by parsers.
    - `ParseResult<N,E>`: to represent either a successful parse of type `N` or `E` as error.
    * **Core Combinators:**
        * `any_token(): Parser<Char>`: Parses any single character.
        * `token(c: Char): Parser<Char>`:  Parses a single character `c`.
        * `literal(p: &str): Parser<&str>`: Parses a specific literal pattern `p`.
        * `ascii_alphabet(): Parser<Char>`: Parse a single valid ascii alphabet.
        * `map<N, NT>(self, f: (N) -> NT): Parser<NT>`:  Transforms the result of `self` (current parser) using a function `f`.
        * `or(self, alternate_parser: Parser<F>): Parser<F>`: Tries `self` first; if it fails, tries `alternate_parser` and returns error if either fails.
        * `repeat_zero_or_more_times(self): Parser<List<N>>`: Parses zero or more repetitions of current `parser`.
        * `repeat_one_or_more_times(self): Parser<List<N>>`: Parses one or more repetitions of current parser `parser`. 

    * **Testing:**
        Create simple test cases to verify that core combinators work as expected.

        * `any_token()` should succeed on any single character but fails on End Of Input.
        * `token('a')` should succeed on "a" and fail on "b".
        * `literal("hello")` should succeed on "hello" and fail on "hell".
        * `ascii_alphabet()` should succeed on 'a' but fail on '1'.
        * `token('a').map({ it -> it.toUpperCase() })` should parse "a" and produce "A".
        * `token('a').or(token('b'))` should succeed on both "a" and "b" and fail on "c".
        * `token('a').repeat_zero_or_more_times()` should succeed on "aaa", "abc", "bc" and Empty Input "".
        * `token('a').repeat_one_or_more_times()` should succeed on "aaa", "abc", fail on "bc" and Empty Input "".

## Resources
- [The Rust Programming Language](https://www.rust-lang.org/book)
- [Learning Parser combinators with rust](https://bodil.lol/parser-combinators/)
- [Parser Combinators from scratch](https://youtu.be/6oQLRhw5Ah0?si=DbQI2Cvn6FLL-w6U)