# Project Compilers Interpreters

Dit is de code die hoort bij mijn project compilers/interpreters.

## Korisp Grammar

```bnf
Program       -> Expression*
Expression    -> VarExpression | IfExpression | List | Atom

VarExpression -> "(" "var" Symbol Expression ")"

                          condition  then       else
IfExpression  -> "(" "if" Expression Expression Expression? ")"

                     name   arguments
FunctionCall  -> "(" Symbol Expression* ")"

List          -> "(" Expression* ")"
Atom          -> Boolean | Number | String | Symbol | "nil"

Boolean       -> "true" | "false"
Number        -> {digit}+ (. {digit}+)?
String        -> \" {character}* \"
Symbol        -> {character}+
```
