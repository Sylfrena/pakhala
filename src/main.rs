#[allow(dead_code)]
#[allow(unused_imports)]

mod token;
mod lexer;
mod repl;


fn main() { //-> io::Result<()> {
    //let hi: &str = "oj";
    repl::start();

    //let mut buffer = String::new();
    //io::stdin().read_to_string(&mut buffer)?;
    //Ok(())
}
