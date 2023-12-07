fn fizzbuzz () -> i32 {

    let mut fizzbuzz_counter : i32  = 0;

    for i in 1..=301 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizzbuzz");
            fizzbuzz_counter += 1;
        } else if i % 5 == 0 {
            println!("buzz");
        } else if i % 3 == 0 {
            println!("fizz");
        }
    }

    return fizzbuzz_counter;
}


fn main() {

    println!(" Welcome to the solana bootcamp! ");
    
    let fizzbuzz_count = fizzbuzz();

    println!("Number of times 'fizz buzz' occurred: {}", fizzbuzz_count);

    println!("Hello, world!");
}
