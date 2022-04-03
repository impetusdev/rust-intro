fn main() {
    println!("Hello, world!");
    
    println!("{}", another_function(5, 's'));
}

fn another_function(x: i32, unit_label: char) -> String {
    format!("Another function number: {}{}", x, unit_label)
}