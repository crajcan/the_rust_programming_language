use std::cmp::Ordering;

fn main() {
    let x = 5u32;
    let y = 0o77u32;

    match x.cmp(&y) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    let mut tup = (500, 6.4, 1);
    let (x, _y, z) = tup;
    tup.2 = 5;
    println!("third value: {}", tup.2);
    println!("x: {}", x);
    println!("z: {}", z);
    println!("tup: {:?}", tup);

    println!("another: {}", another());
}

fn another() -> i32 {
    15
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    (c * (9.0 / 5.0)) + 32.0
}

fn almost_equal(x: f32, y: f32) -> bool {
    (x - y).abs() < 0.01
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        return 1;
    }

    let mut a = 1;
    let mut b = 1;
    let mut i = 2;

    while i <= n {
        let temp = a + b;
        a = b;
        b = temp;

        i = i + 1;
    }

    b
}

const DAYS: &[&str] = &[
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth", "tenth",
    "eleventh", "twelfth",
];

const GIFTS: &[&str] = &[
    "a partridge in a pear tree",
    "two turtle doves and",
    "three french hens",
    "four calling birds",
    "five golden rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming"
];

fn twelve_days() {
    for day in 0..12 {
        println!("On the {} day of Christmas my true love gave to me", DAYS[day]);
        for gift in (0..day+1).rev() {
            println!("{}", GIFTS[gift]);
        }
        println!("");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_almost_equal() {
        assert!(almost_equal(5.0, 5.00));
        assert!(almost_equal(5.0, 5.001));
        assert!(almost_equal(5.0, 5.009));
        assert!(!almost_equal(5.0, 5.01));
        assert!(!almost_equal(5.0, 5.10));
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert!(almost_equal(fahrenheit_to_celsius(32.0), 0.0));
        assert!(almost_equal(fahrenheit_to_celsius(212.0), 100.0));
        assert!(almost_equal(fahrenheit_to_celsius(100.0), 37.77));
        assert!(almost_equal(fahrenheit_to_celsius(-5.0), -20.56));
        assert!(almost_equal(fahrenheit_to_celsius(20.0), -6.66));
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert!(almost_equal(celsius_to_fahrenheit(0.0), 32.0));
        assert!(almost_equal(celsius_to_fahrenheit(100.0), 212.0));
        assert!(almost_equal(celsius_to_fahrenheit(37.777777), 100.0));
        assert!(almost_equal(celsius_to_fahrenheit(-20.56), -5.0));
        assert!(almost_equal(celsius_to_fahrenheit(-6.666666), 20.0));
    }

    #[test]
    fn test_fib() {
        assert_eq!(fib(0), 1);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 2);
        assert_eq!(fib(3), 3);
        assert_eq!(fib(4), 5);
        assert_eq!(fib(5), 8);
    }

    #[test]
    fn test_twelve_days() {
        twelve_days();
    }
}
