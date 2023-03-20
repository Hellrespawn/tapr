# `korisp`

Dit is een interpreter voor mijn _LISP_-achtige programmeertaal `korisp`. Deze gemaakt voor als onderdeel van de vrije-keuze ruimte van de opleiding AD Software Development.

## Vereisten

De minimale versie is Rust 1.65.

Om de AST te laten af te beelden is ook _GraphViz_ vereist.

## Installatie

`korisp` kan geïnstalleerd worden door middel van `cargo install`.

## Uitvoeren

Na installatie kan `korisp` op de command line uitgevoerd worden.

Indien installatie niet gewenst is, kan deze ook uitgevoerd worden met behulp van `cargo run`. Let er dan op dat voor `korisp` na een `--` verschijnen, bijv. `cargo run -- file.ksp`.

- `korisp`: Zonder argumenten start de REPL (*R*ead-*E*valuate-*P*rint *L*oop), een interactieve shell.

- `korisp file.ksp`: Voert het gegeven bestand uit.

## Debugging

Er zijn een aantal _environment variables_ beschikbaar.

- `DEBUG_AST`: Als deze `1` is, dan wordt de AST gevisualizeerd.
- `DEBUG_TOKENS`: Als deze `1`, dan worden tokens geprint op de command line.

Ook kan de functie `_env()` uitgevoerd worden om de huidige environment te printen.
