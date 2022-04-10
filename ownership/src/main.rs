fn main() {
    let mut s = String::from("hello");

    // println!("{}", s);
    change(&mut s);
    // println!("{}", s);
}

fn change(some_string: &mut String) {
    println!("{}", some_string);
}
