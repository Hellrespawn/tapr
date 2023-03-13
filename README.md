# `korisp`

Dit is een interpreter voor mijn _LISP_-achtige programmeertaal `korisp`. Deze gemaakt voor als onderdeel van de vrije-keuze ruimte van de opleiding AD Software Development.

## Vereisten

De minimale versie is Rust 1.65.

## Installatie

`korisp` kan geïnstalleerd worden door middel van `cargo install`.

## Uitvoeren

Na installatie kan `korisp` op de command line uitgevoerd worden.

Indien installatie niet gewenst is, kan deze ook uitgevoerd worden met behulp van `cargo run`. Let er dan op dat voor `korisp` na een `--` verschijnen, bijv. `cargo run -- file.ksp`.

- `korisp`: Zonder argumenten start de REPL (*R*ead-*E*valuate-*P*rint *L*oop), een interactieve shell.

- `korisp file.ksp`: Voert het gegeven bestand uit.
