

fn main() {
    println!("Hello, world!");

    let mut number = String::new();
    println!("Put in a number: ");
    std::io::stdin().read_line(&mut number);

    let num = number.trim().parse::<u32>().expect("failed to turn into number");

    for i in 1..=num {
        match i {
            i if (i % 15) == 0 => fizz_buzz(i),
            i if (i % 3) == 0 => fizz(i),

            i if (i % 5) == 0 => buzz(i),
            _ => println!("{i}"),
        }
    }

}


fn fizz (num: u32) {
    println!("Fizz {}", num);
}

fn buzz (num: u32) {
    println!("Buzz {}", num);
}

fn fizz_buzz (num: u32) {
    println!("FizzBuzz {}", num);
}
