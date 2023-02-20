# Project Compilers Interpreters

Dit is de code die hoort bij mijn project compilers/interpreters.

## Korisp Grammar

```bnf
Program             -> Expression

Expression          -> SetExpression
                     | IfExpression
                     | WhileExpression
                     | FunctionCall
                     | DataType

                                                   scope
SetExpression       -> "(" "set" "(" Variables ")" Expression ")"

                       name   value
Variables           -> Variable
                     | ( "(" Variable ")" )+

Variable            -> Symbol Expression

                                condition  then       else
IfExpression        -> "(" "if" Expression Expression Expression? ")"

                                   condition  then
WhileExpression     -> "(" "while" Expression Expression ")"

                           name   arguments
FunctionCall        -> "(" Symbol Expression* ")"

DataType            -> "'"? (List | Atom)

List                -> "(" Expression* ")"

Atom                -> "nil" | Boolean | Number | String | Symbol

Boolean             -> "true" | "false"
Number              -> {digit}+ (. {digit}+)?
String              -> \" {character}* \"
Symbol              -> {character}+
```
