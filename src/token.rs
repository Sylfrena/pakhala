
pub type TokenType<'a> = &'a str;

pub struct Token <'a> {
    pub token_type: TokenType<'a>,
    pub literal: &'a str,
}

impl Token <'_> {
    pub fn new_token<'a>(tok_type: TokenType<'a>, ch: &'a str) -> Token<'a> {
        Token{token_type:tok_type, literal: ch}
    }
}

pub const ILLEGAL: TokenType = "ILLEGAL";
pub const EOF: TokenType = "EOF";

//Identifiers

pub const IDENT: TokenType = "IDENTIFIER";
pub const INT: TokenType = "INT";
pub const FLOAT: TokenType = "FLOAT";

// Operators
pub const ASSIGN: TokenType = "= " ;
pub const PLUS: TokenType = "+" ;
pub const MINUS: TokenType = "-" ;
pub const DIVIDE: TokenType = "/" ;
pub const MULTIPLY: TokenType = "*" ;
pub const MODULO: TokenType = "%" ;
//EQUAL_TO: TokenType = '= '
pub const GREATER_THAN: TokenType = ">" ;
pub const LESS_THAN: TokenType = "<" ;
//#UNEQUAL_TO: TokenType = '!
//#AND: TokenType = '&&'
//#OR: TokenType = '||'
pub const NOT: TokenType = "!" ;
  
//# Delimiters
pub const COMMA: TokenType = "," ;
pub const SEMICOLON: TokenType = ";" ;
pub const QUESTION: TokenType = "?" ;
  
pub const LPAREN: TokenType = "(" ;
pub const RPAREN: TokenType = ")" ;
pub const LSQUARE: TokenType = "[" ;
pub const RSQUARE: TokenType = "]" ;
pub const LCURLY: TokenType = "{" ;
pub const RCURLY: TokenType = "}" ;

//Keywords
pub const FUNCTION: TokenType = "def" ;
pub const LET: TokenType = "let";
//SAY: TokenType ='SAY'

