#[macro_use]
extern crate lazy_static;
extern crate regex;

mod convert;
mod prompt;

fn main() {
    let user_input = prompt::for_input("Type a sentence to convert to pig latin");
    let converted_text = convert::to_pig_latin(&user_input);

    println!("");
    println!("{}", converted_text);
}
