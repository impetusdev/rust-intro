fn main() {
    let hello = String::from("Hello, world!");
    
    let word = first_word(&hello);

    println!("word is {}", word);
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        println!("current item is: {}", &item );
         if item == b' ' {
            println!("i is: {}", i );
             
            return i;
        }
    }
    
    s.len()
}