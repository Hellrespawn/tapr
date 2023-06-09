# KorISP

**NOTE**: This project is published for reference only. There is no license and all rights are reserved.

This is an interpreter for a _Lisp_-style programming language called KorISP.

This project was originally created for an elective course on compilers and interpreters of the Associate Degree in Software Development at the Amsterdam University of Applied Sciences.

## Requirements

- Rust (MSRV: 1.65)
- GraphViz (Optional, for visualizing Abstract Syntax Trees)

## Installation

1. Ensure `cargo` and Cargo's `bin` folder are on your `PATH`.
1. Clone the repository.
1. Run `cargo install --path korisp`.

## Usage

Running `korisp` without arguments starts the REPL. An empty line or Ctrl-C will exit.

`korisp <filename>` will run the specified file.

Use `(eval (read-file "stdlib.ksp"))` to import the standard library.

## Debugging

There are two environment variables:

- `DEBUG_AST`: If set and not empty, will visualize the Abstract Syntax Tree of the program using GraphViz.
- `DEBUG_TOKENS`: If set and not empty, will print scanned tokens on the command line.

Within the REPL or programs, the `(_env)`-function, will print the interpreter's current environment.
