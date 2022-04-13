fn main() {
    // Count from 0 to 100;
    
    sum_values(10);
}


fn sum_values (num: u32) -> u32{
    let mut sum = 0; 
    
    for i in 1..num+1 {
        sum += i;
        println!("sum:{}", sum);
    }

    return sum;
}
