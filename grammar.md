# Polytype Parser Grammar 

[Modified from Joshua Lapacik's work here](https://github.com/jlapacik/ftel/blob/master/antlr/flux.g4)

```
'?' = optional; once or none 
'*' = zero or more times
'|' = or 
(values) = a list of possibly repeating values
single quotes indicate a literal value


polytype = 'forall' '[' vars? ']' ('where' constraints)? monotype

vars = tvr (',' tvr)* 
constraints = constraint ( (',' | 'and') constraint)* 
constraint = tvr ('is' | ':') typeClass 
typeClass = IDENTIFIER 
monotype = tvr | typ | arr | obj | fun

tvr = '\'' IDENTIFIER 
typ = INT | FLT | STR | BOOL | REGEXP  
arr = '[' monotype ']'
obj = '{' properties? '}'
fun = '(' arguments? ')' '->' monotype
properties = property ( '|' property )* ( '|' tvr)?
property = IDENTIFIER ':' monotype
arguments = argument ( ',' argument )*
argument = required | optional | pipe
required = IDENTIFIER ':' monotype
optional = '?' IDENTIFIER ':' monotype
pipe = '<-' IDENTIFIER? ':' monotype

INT = 'int'
FLT = 'flt'
STR = 'str'
BOOL = 'bool'
REGEXP = 'regexp'
IDENTIFIER = LETTER (LETTER | [0-9])*
LETTER = [a-zA-Z0-9]
WS = [ \t\r\n]+ -> skip
```