fn first_word(s: &String) -> &str {
    let bytess = s.as_bytes();

    for (i, &item) in bytess.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn main() {
    let phrase = String::from("ping pong");

    let word = first_word(&phrase);

    println!("\"{}\": {}",&phrase,&word);
}
