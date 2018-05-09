use parse;

fn main() {
    println!("{:?}", parse::get_tokens("(def a 20)".to_owned()));
    println!("Hello, world!");
}
