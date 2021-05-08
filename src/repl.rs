
extern crate scanner_rust;

use crate::token;
use crate::lexer;
use std::io;
use std::io::Write;

use scanner_rust::Scanner;

const PROMPT: &str = ">>";

fn rep(inp: String) {
    let mut l = lexer::Lexer::new(&inp);
    let mut tok: token::Token;


    loop {
        //println!("I entered loop, gonna next token");
        tok = l.next_token();
        //println!("I should have next tokened");
        
        println!("{{Type: {}, Literal: {}}}", tok.token_type, tok.literal);
        if tok.token_type == token::EOF {
        //    println!("gonna break");
            break
        }
        l.read_char();          
    }
}

fn display_prompt() {
    println!("{}", PROMPT);
}

pub fn start() {
    
    println!("***************************************************************");
    println!("Welcome to the pakhala REPL");

    io::stdout().flush().unwrap();
    let mut sc = Scanner::new(io::stdin());

    loop {
        //me was using next_isize(). ofc no wonder it worked only for integers.
        match sc.next_line() {              
            Ok(i) => { 
                let s = i.unwrap_or("0".to_string()).to_string();
                if s.len() > 0 {
                    //println!("I am the string {} ", s);
                                  
                    match s.ends_with(";;") {
                        true => {
                            display_prompt();
                            //println!("\n Here you go!\n");
                            rep(s.split_terminator(";;").collect())
                        }
                        _ => {
                            //println!("\n Here you go!\n"); 
                            rep(s);
                            display_prompt();
                        }
                    }
                } else {
                    display_prompt()
                }
            },
            Err(_) => { display_prompt();
                        io::stdout().flush().unwrap();
                      }
        }
    }


}