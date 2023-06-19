# Tapr

Tapr (tapper) is a general-purpose LISP-style programming language.

## Requirements

- Rust (MSRV: 1.70)
- GraphViz (Optional, for visualizing Abstract Syntax Trees)

## Installation

1. Ensure `cargo` and Cargo's `bin` folder are on your `PATH`.
1. Clone the repository.
1. Run `cargo install --path tapr`.

## Usage

Running `tapr` without arguments starts the REPL. An empty line or Ctrl-C will exit.

`tapr <filename>` will run the specified file.

Use `(import stdlib)` to import the standard library.

## Debugging

There are two environment variables:

- `DEBUG_AST`:    If set and not empty, will visualize the Abstract Syntax Tree of the program using GraphViz. Will create an image per parsed file or line. New REPL-sessions will overwrite old files. Will retain created .dot-files when set to "dot".

- `DEBUG_PARSER`: If set and not empty, will print all Pairs parsed by `pest`.

Within the REPL or programs, the `(_env)`-function, will print the interpreter's current environment.
