# Project Compilers Interpreters

Dit is de code die hoort bij mijn project compilers/interpreters.

## Korisp Grammar

```bnf
Program    -> List*
List       -> "(" (Atom | List) * ")"
Atom       -> Number | String | Symbol | nil

Number     -> {digit}+
String     -> \" {character}* \"
Symbol     -> {character}+
```
