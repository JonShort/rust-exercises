use std::io;

pub fn for_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut response = String::new();

    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read input");

    String::from(response.trim())
}
