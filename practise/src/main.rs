fn main() {
    // println!("{}", convert_f_to_c(32.)); // should be 0;
    // println!("{}", convert_f_to_c(100.)); // should be 37.7778;
    generate_fib(1000);

}

fn convert_f_to_c(f_temp: f32) -> f32{
    (f_temp - 32.) * 5./9.
}

fn generate_fib(j: i32) {
    let mut count = 0;
    let mut i: i64 = 1;
    let mut prev: i64 = 1;
    println!("{}", i);


    while count < j {
        let held: i64 = i;
        i = i + prev;
        prev = held;
        println!("{}", i);
        count += 1;
    }
}
// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

//first line  

// 