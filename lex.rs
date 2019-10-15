use std::{fmt};

type Token int 

pub enum TokenTypes (
    ERROR,
    EOF,

    // Keywords
    FORALL,
    WHERE,

    // Other stuff? 
    CONSTRAINT,
    TYPEVAR,

    ARRAY,
    OBJECT,
    FUNCTION,
    PROPERTY,
    REQUIREDARG,
    OPTIONALARG,
    PIPEARG,

    // Literals and Indentifiers
    INT,
    FLOAT,
    STRING,
    BOOL,
    REGEX,
    IDENTIFIER,
    LETTER,
    WHITESPACE,

    // Operators
    LEFTBRACKET,
    RIGHTBRACKET,
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
)

impl fmt::Display for TokenTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenTypes::ERROR => f.write_str("error"),
            TokenTypes::EOF => f.write_str("eof"),
            TokenTypes::FORALL => f.write_str("forall"),
            TokenTypes::WHERE => f.write_str("where"),
            TokenTypes::CONSTRAINT => f.write_str("contraint"),
            TokenTypes::TYPEVAR => f.write_str("typeVar"),
            TokenTypes::ARRAY => f.write_str("array"),
            TokenTypes::OBJECT => f.write_str("object"),
            TokenTypes::FUNCTION => f.write_str("function"),
            TokenTypes::PROPERTY => f.write_str("property"),
            TokenTypes::REQUIREDARG => f.write_str("requiredArg"),
            TokenTypes::OPTIONALARG => f.write_str("optionalArg"),
            TokenTypes::PIPEARG => f.write_str("pipeArg"),
            TokenTypes::INT => f.write_str("int"),
            TokenTypes::FLOAT => f.write_str("float"),
            TokenTypes::STRING => f.write_str("string"),
            TokenTypes::BOOL => f.write_str("bool"),
            TokenTypes::REGEX => f.write_str("regex"),
            TokenTypes::IDENTIFIER => f.write_str("identifier"),
            TokenTypes::LETTER => f.write_str("letter"),
            TokenTypes::WHITESPACE => f.write_str("whitespace"),
            TokenTypes::LEFTBRACKET => f.write_str("leftbracket"),
            TokenTypes::RIGHTBRACKET => f.write_str("rightbracket"),
            TokenTypes::LEFTPAREN => f.write_str("leftparen"),
            TokenTypes::RIGHTPAREN => f.write_str("rightparen"),
            TokenTypes::LEFTBRACE => f.write_str("leftbrace"),
            TokenTypes::RIGHTBRACE => f.write_str("rightbrace")
        }
    }
}


pub fn Lex(input string): (Sender<>, Receiver<>) 