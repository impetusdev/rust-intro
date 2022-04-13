fn main() {
    let s = String::from("Hello, world!");
    
    let hello = first_word(&s);

    println!("slice of word is {}", hello);
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
    
}