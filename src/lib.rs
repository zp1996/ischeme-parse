mod parse;

use parse::*;

fn main() {
    println!("{:?}", get_tokens("(def a 20)".to_owned()));
    println!("Hello, world!");
}
