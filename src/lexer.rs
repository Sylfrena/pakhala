#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]

use crate::token;
use std::convert::TryInto;
use std::error;
use std::iter::Iterator;
use std::str::from_utf8;

//type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

//struct InvalidToken;

struct Lexer <'a> {
    input: &'a str,
    position: i32,
    read_position: i32,
    ch: &'a str,
}

impl Lexer <'_> {
   fn new(input: &str) -> Lexer <'_> {
       let mut l = Lexer{
             input: input,
             position: 0,
             read_position: 0,
             ch: "\0"};
        l.read_char();
        l
   }

   fn read_char(&mut self) {
       if self.read_position >= (self.input).len().try_into().unwrap() {
           println!("ghusraha hai");
           self.ch = "0";
       } else {
           let ch_byte = self.input.as_bytes();
           let mut pos = self.read_position as usize;
           self.ch = from_utf8(&ch_byte[pos..pos+1]).unwrap();
           println!("ghusraha hai at pos {}",pos);
       }
       println!("moi her too");
       self.position = self.read_position;
       self.read_position += 1;

   }


   fn next_token(&mut self) -> token::Token {
      println!("self.ch is{}", self.ch);
      let mut tok = match self.ch {
           "-" => token::Token::new_token(token::MINUS, self.ch),
           ";" => token::Token::new_token(token::SEMICOLON, self.ch),
           "(" => token::Token::new_token(token::LPAREN, self.ch),
           ")" => token::Token::new_token(token::RPAREN, self.ch),
           "," => token::Token::new_token(token::COMMA, self.ch),
           "+" => token::Token::new_token(token::PLUS, self.ch),
           "0" => token::Token::new_token(token::EOF, ""),
           _ => token::Token::new_token(token::ILLEGAL, ""),
        };

        self.read_char();
        tok
   }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_lexer() {
        let input = "-+(),;" ;

        struct Expected <'a> {
            expected_type: token::TokenType <'a>,
            expected_literal: &'a str,
        }

        let test_v = vec![
            Expected{expected_type: token::MINUS, expected_literal: "-"},
            Expected{expected_type: token::PLUS, expected_literal: "+"},
            Expected{expected_type: token::LPAREN, expected_literal: "("},
            Expected{expected_type: token::RPAREN, expected_literal: ")"},
            Expected{expected_type: token::COMMA, expected_literal: ","},
            Expected{expected_type: token::SEMICOLON, expected_literal: ";"},
            Expected{expected_type: token::EOF, expected_literal: ""}
        ];


        let mut l = Lexer::new(input);

        for tt in test_v{
            let tok: token::Token = l.next_token();

            println!("Type guessed by program \t Actual type");
            println!("{}\t{}", tok.token_type, tt.expected_type);
            println!("{}now this\t{}", tok.literal, tt.expected_literal);

            assert_eq!(tok.token_type, tt.expected_type);
            assert_eq!(tok.literal, tt.expected_literal);
        }

    }
}
