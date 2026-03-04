fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);

    println!("{word}");
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello} {world}");
    let slice = &s[3..s.len()];
    println!("{slice}");
    let string_literal = "string literal";
    let word = first_word(string_literal);
    println!("{string_literal} {word}");
    let a = [1, 2, 3, 4, 5];
    for i in a {
        println!("{i}");
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
