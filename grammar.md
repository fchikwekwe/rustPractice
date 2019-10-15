# Polytype Parser Grammar 

[Modified from Joshua Lapacik's work here](https://github.com/jlapacik/ftel/blob/master/antlr/flux.g4)

```
'?' = optional; once or none 
'*' = zero or more times
'|' = or 
(values) = a list of possibly repeating values
single quotes indicate a literal value


polytype    = 'forall' '[' vars? ']' ('where' constraints)? monotype

vars        = typeVar (',' typeVar)* 
constraints = constraint ( (',' | 'and') constraint)* 
constraint  = typeVar ('is' | ':') typeClass 
typeClass   = IDENTIFIER 
monotype    = typeVar | typ | array | object | function

typeVar     = '\'' IDENTIFIER 
typ         = INT | FLOAT | STRING | BOOL | REGEX 
array       = '[' monotype ']'
object      = '{' properties? '}'
function    = '(' arguments? ')' '->' monotype
properties  = property ( '|' property )* ( '|' typeVar)?
property    = IDENTIFIER ':' monotype
arguments   = argument ( ',' argument )*
argument    = required | optional | pipe
required    = IDENTIFIER ':' monotype
optional    = '?' IDENTIFIER ':' monotype
pipe        = '<-' IDENTIFIER? ':' monotype

INT         = 'int'
FLT         = 'float'
STR         = 'string'
BOOL        = 'bool'
REGEX       = 'regular expression'
IDENTIFIER  = LETTER (LETTER | [0-9])*
LETTER      = [a-zA-Z0-9]
WS          = [ \t\r\n]+ -> skip
```