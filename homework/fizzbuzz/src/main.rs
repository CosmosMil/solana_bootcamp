fn main() {
    println!("Welcome!");
    fizz_buzz();
}

fn fizz_buzz() {
    let mut count = 0;

    for n in 1..301 {
        if n % 3 == 0 && n % 5 == 0 {
            println!("fizz buzz");
            count += 1;
        } else if n % 3 == 0 {
            println!("fizz")
        } else if n % 5 == 0 {
            println!("buzz")
        }
    }
    println!("fizzbuzz occurred {} times", count);
}
