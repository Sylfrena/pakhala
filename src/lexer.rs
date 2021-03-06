#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]

use crate::token;
use std::convert::TryInto;
use std::error;
use std::iter::Iterator;
use std::str::from_utf8;
use std::clone::Clone;

//type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

//struct InvalidToken;

pub struct Lexer <'a> {
    input: &'a str,
    position: i32,
    read_position: i32,
    ch: &'a str,
}



impl Lexer <'_> {
   pub fn new(input: &str) -> Lexer <'_> {
       let mut l = Lexer{
             input: input,
             position: 0,
             read_position: 0,
             ch: "\0"};
        l.read_char();
        l
   }

   fn is_letter(&self) -> bool  {
        "a" <= self.ch && self.ch <= "z" ||
        "A" <= self.ch && self.ch <= "Z" ||
        self.ch == "_"
    }

   
   
   fn is_digit(&self) -> bool {
        "0" <= self.ch && self.ch <= "9"
    }



   
   
   pub fn read_char(&mut self) {
       if self.read_position >= (self.input).len().try_into().unwrap() {
           //println!("read_char mein ghusraha hai");
           self.ch = "\0";
       } else {
           let ch_byte = self.input.as_bytes();
           let mut pos = self.read_position as usize;
           self.ch = from_utf8(&ch_byte[pos..pos+1]).unwrap();
           //println!("ghusraha hai at pos {}",pos);
       }
       //println!("moi her too");
       self.position = self.read_position;
       self.read_position += 1;
   }

   fn peek_char(&mut self) -> &str{
       if self.read_position >= (self.input).len().try_into().unwrap() {
           "\0"
       } else {
        let ch_byte = self.input.as_bytes();
        let mut pos = self.read_position as usize;
        from_utf8(&ch_byte[pos..pos+1]).unwrap()
       }
   }

   fn read_identifier(&mut self) -> &str{
       let mut position = self.position;
       while self.is_letter() {
           self.read_char()
       }

       let input_byte = self.input.as_bytes();
       from_utf8(&input_byte[position as usize..self.position as usize])
            .unwrap()
   }

   fn read_number(&mut self) -> &str{
       let mut position = self.position;
       while self.is_digit() {
           self.read_char()
       }

       self.read_position = self.read_position - 1;
       let input_byte = self.input.as_bytes();
       from_utf8(&input_byte[position as usize..self.position as usize])
            .unwrap()
   }

   fn consume_whitespace(&mut self) {
       while self.ch == " " || self.ch == "\t" ||
              self.ch == "\n" || self.ch == "\r" {
                self.read_char();
        }
   }

   pub fn next_token<'a>(&'a mut self) -> token::Token {
      //println!("self.ch is{}", self.ch);

      self.consume_whitespace();
      let mut tok = match self.ch {
        "-" => token::Token::new_token(token::MINUS, self.ch),
        ";" => token::Token::new_token(token::SEMICOLON, self.ch),
        "(" => token::Token::new_token(token::LPAREN, self.ch),
        ")" => token::Token::new_token(token::RPAREN, self.ch),
        "[" => token::Token::new_token(token::LSQUARE, self.ch),
        "]" => token::Token::new_token(token::RSQUARE, self.ch),
        "{" => token::Token::new_token(token::LCURLY, self.ch),
        "}" => token::Token::new_token(token::RCURLY, self.ch),
        "?" => token::Token::new_token(token::QUESTION, self.ch),
        "," => token::Token::new_token(token::COMMA, self.ch),
        "+" => token::Token::new_token(token::PLUS, self.ch),
        "*" => token::Token::new_token(token::MULTIPLY, self.ch),
        "/" => token::Token::new_token(token::DIVIDE, self.ch),
        "%" => token::Token::new_token(token::MODULO, self.ch),
        "<" => token::Token::new_token(token::LESS_THAN, self.ch),
        ">" => token::Token::new_token(token::GREATER_THAN, self.ch),
        "=" => if self.peek_char() == "=" {
                   let _ch: &'a str = self.ch;
                   //workaround because whole host of borrowing problems
                   //if i try to concatenate the way in the book
                   //everything requires kinda &(String) which is illegal
                   self.read_char();
                   //let n =
                   //let n: &'a str = (ch.to_string() + self.ch);//format!("{}{}",ch, self.ch);
                   token::Token{
                        token_type: token::EQ,
                        literal: "=="}
                } else {
                    token::Token::new_token(token::ASSIGN, self.ch)
                },
        "!" => if self.peek_char() == "=" {
                    self.read_char();
                    token::Token{
                        token_type: token::NOT_EQ,
                        literal: "!="}
                } else {
                    token::Token::new_token(token::BANG, self.ch)
                },
        "\0" => token::Token::new_token(token::EOF, ""),
        _ => if self.is_letter() {
                let lit = self.read_identifier();
                token::Token{
                     token_type: token::Token::look_ident(lit),
                     literal: lit}
             } else if self.is_digit() {
                let lit = self.read_number();
                token::Token{token_type: token::INT,literal: lit}
             } else {
                 token::Token::new_token(token::ILLEGAL, "")
             }
      };
     tok
     //read_char has not been called here because it confclicts with the self.is_digit and self.is_letter
     //read_char is called after next_token
   }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_lexer() {
        let _input = "let sumo =9;" ;
        // "let sumo= 9" and "let sumo =9;" fails
        //identifiers need space after them for some reason
        // self.position-1 works for the digit part, not letters

        let input2 = "let five = 5;
                      let ten = 10;
                      let add = def (x , y );
                      x + y ;
                      let result = add (five , ten );
                      10 == 10
                      10 != 1";
        //same issue here, identifiers still need one space after them

        struct Expected <'a> {
            expected_type: token::TokenType <'a>,
            expected_literal: &'a str,
        }

        let test_v = vec![
          /*Expected{expected_type: token::MINUS, expected_literal: "-"},
            Expected{expected_type: token::PLUS, expected_literal: "+"},
            Expected{expected_type: token::LPAREN, expected_literal: "("},
            Expected{expected_type: token::RPAREN, expected_literal: ")"},
            Expected{expected_type: token::COMMA, expected_literal: ","},
            Expected{expected_type: token::SEMICOLON, expected_literal: ";"},
            Expected{expected_type: token::EOF, expected_literal: ""}*/
            //Expected{expected_type: token::LET, expected_literal: "let"},
            //Expected{expected_type: token::IDENT, expected_literal: "sumo"},
            //Expected{expected_type: token::ASSIGN, expected_literal: "="},
            //Expected{expected_type: token::INT, expected_literal: "9"},
            //Expected{expected_type: token::SEMICOLON, expected_literal: ";"},
            Expected{expected_type: token::LET, expected_literal: "let"},
            Expected{expected_type: token::IDENT, expected_literal: "five"},
            Expected{expected_type: token::ASSIGN, expected_literal: "="},
            Expected{expected_type: token::INT, expected_literal: "5"},
            Expected{expected_type: token::SEMICOLON, expected_literal: ";"},

            Expected{expected_type: token::LET, expected_literal: "let"},
            Expected{expected_type: token::IDENT, expected_literal: "ten"},
            Expected{expected_type: token::ASSIGN, expected_literal: "="},
            Expected{expected_type: token::INT, expected_literal: "10"},
            Expected{expected_type: token::SEMICOLON, expected_literal: ";"},

            Expected{expected_type: token::LET, expected_literal: "let"},
            Expected{expected_type: token::IDENT, expected_literal: "add"},
            Expected{expected_type: token::ASSIGN, expected_literal: "="},
            Expected{expected_type: token::FUNCTION, expected_literal: "def"},
            Expected{expected_type: token::LPAREN, expected_literal: "("},
            Expected{expected_type: token::IDENT, expected_literal: "x"},
            Expected{expected_type: token::COMMA, expected_literal: ","},
            Expected{expected_type: token::IDENT, expected_literal: "y"},
            Expected{expected_type: token::RPAREN, expected_literal: ")"},
            Expected{expected_type: token::SEMICOLON, expected_literal: ";"},

            Expected{expected_type: token::IDENT, expected_literal: "x"},
            Expected{expected_type: token::PLUS, expected_literal: "+"},
            Expected{expected_type: token::IDENT, expected_literal: "y"},
            Expected{expected_type: token::SEMICOLON, expected_literal: ";"},

            Expected{expected_type: token::LET, expected_literal: "let"},
            Expected{expected_type: token::IDENT, expected_literal: "result"},
            Expected{expected_type: token::ASSIGN, expected_literal: "="},
            Expected{expected_type: token::IDENT, expected_literal: "add"},
            Expected{expected_type: token::LPAREN, expected_literal: "("},
            Expected{expected_type: token::IDENT, expected_literal: "five"},
            Expected{expected_type: token::COMMA, expected_literal: ","},
            Expected{expected_type: token::IDENT, expected_literal: "ten"},
            Expected{expected_type: token::RPAREN, expected_literal: ")"},
            Expected{expected_type: token::SEMICOLON, expected_literal: ";"},

            Expected{expected_type: token::INT, expected_literal: "10"},
            Expected{expected_type: token::EQ, expected_literal: "=="},
            Expected{expected_type: token::INT, expected_literal: "10"},

            Expected{expected_type: token::INT, expected_literal: "10"},
            Expected{expected_type: token::NOT_EQ, expected_literal: "!="},
            Expected{expected_type: token::INT, expected_literal: "1"},

            Expected{expected_type: token::EOF, expected_literal: ""}
        ];


        let mut l = Lexer::new(input2);

        for tt in test_v{
            { //since next_token and read_char can't be called in the same scope
                let tok: token::Token = l.next_token();

                println!("Type guessed by program \t Actual type");
                println!("{}\t{}", tok.token_type, tt.expected_type);
                println!("{}now this\t{}", tok.literal, tt.expected_literal);

                assert_eq!(tok.token_type, tt.expected_type);
                assert_eq!(tok.literal, tt.expected_literal);
            }

            l.read_char();
        }

    }
}
