#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]

use crate::token;
use crate::lexer;

pub trait Node {
    fn token_literal(&self) -> &str;
}

pub trait Statement: Node {
    fn statement_node();
}

pub trait Expression: Node {
    fn expression_node();
}

pub struct Program<T: Statement> {
    statements: [T], //also, should I be using vector here? when to use what?
}

pub struct Identifier<'a> {
    token: token::Token<'a>,
    value: String,
}

pub struct LetStatement<'a> {
    token: token::Token<'a>,
    name: Identifier<'a>,
    //value: Expression,
}

//okay impl<T> for Program<T> didn't work here. Why?
//but then this is not trait implementation any more huh
//listing 17-6 in th rust book
impl<T: Statement> Program <T> 
{   //is this the best way to do a recursive piece in rust idiomatically?
    fn token_literal(&self) -> &str {
        if (self.statements).len() > 0 {
            self.statements[0].token_literal()
        }
        else {
            ""
        }
    }        
} 

impl Statement for LetStatement<'_> {
    fn statement_node() {}
}

impl Node for LetStatement<'_> {
    fn token_literal(&self) -> &str {
        self.token.literal
    }
}

impl Expression for Identifier<'_> {
    fn expression_node() {}
}

impl Node for Identifier<'_> {
    fn token_literal(&self) -> &str {
        self.token.literal
    }
}


//lots of redundancy here ugh

