# Project Compilers Interpreters

Dit is de code die hoort bij mijn project compilers/interpreters.

## Korisp Grammar

```bnf
Expression          -> Define
                     | If
                     | While
                     | Lambda
                     | Call
                     | QuotedDatum
                     | Datum

                                 name   value
Define              -> "(" "def" Symbol Expression ")"

                                condition  then       else
If                  -> "(" "if" Expression Expression Expression? ")"

                                   condition  then
While               -> "(" "while" Expression Expression ")"

Lambda              -> "(" "lambda" "(" Symbol* ")" Expression ")"

                           name   arguments
Call                -> "(" Symbol Expression* ")"

QuotedDatum         -> "(" "quote" Datum ")"
                     |  "'" Datum

Datum               -> List
                     | Atom

List                -> "(" Expression* ")"

Atom                -> "nil" | Boolean | Number | String | Symbol

Boolean             -> "true" | "false"
Number              -> {digit}+ ( . {digit}* )?
String              -> \" {character}* \"
Symbol              -> {character}+
```
