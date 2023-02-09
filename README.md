# Project Compilers Interpreters

Dit is de code die hoort bij mijn project compilers/interpreters.

## Korisp Grammar

```bnf
Program    -> Expression
Expression -> "(" Symbol (Atom | List)* ")"
List       -> "(" (Atom | List) * ")"
Atom       -> Number | String | Symbol

Number     -> {digit}+ (. {digit}+)?
String     -> \" {character}* \"
Symbol     -> {character}+
```
