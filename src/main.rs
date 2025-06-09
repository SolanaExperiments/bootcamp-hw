fn main() {
    println!("Welcome to the Encode Bootcamp FizzBuzz Challenge!");
    fizz_buzz();
}

fn fizz_buzz() {
    let mut fizz_buzz_count = 0;

    for i in 1..=301 {
        if i % 15 == 0 {
            println!("fizz buzz");
            fizz_buzz_count += 1;
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }

    println!("Total number of 'fizz buzz' occurrences: {}", fizz_buzz_count);
}
