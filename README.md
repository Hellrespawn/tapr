# Project Compilers Interpreters

Dit is de code die hoort bij mijn project compilers/interpreters.

## Korisp Grammar

```bnf
Program             -> Expression

Expression          -> SetExpression
                     | IfExpression
                     | WhileExpression
                     | FunctionDefinition
                     | FunctionCall
                     | DataType

                                 name   value
SetExpression       -> "(" "set" Symbol Expression ")"

                                condition  then       else
IfExpression        -> "(" "if" Expression Expression Expression? ")"

                                   condition  then
WhileExpression     -> "(" "while" Expression Expression ")"

                                 name
FunctionDefinition  -> "(" "def" Symbol "(" Parameters ")" Expression ")"

                           name   arguments
FunctionCall        -> "(" Symbol Expression* ")"


Parameters          -> Symbol | "(" Symbol+ ")"

DataType            -> "'"? (List | Atom)

List                -> "(" Expression* ")"

Atom                -> "nil" | Boolean | Number | String | Symbol

Boolean             -> "true" | "false"
Number              -> {digit}+ (. {digit}+)?
String              -> \" {character}* \"
Symbol              -> {character}+
```
