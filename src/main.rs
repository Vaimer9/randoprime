#![feature(map_first_last)]

use std::collections::BTreeSet;

fn is_prime(num: usize) -> bool {
    (2..(num as f64).sqrt().floor() as usize)
        .skip(1)
        .fold(
            true,
            |_, x| (num % x != 0)
        )
}

fn findallprimes(num: usize) -> BTreeSet<usize> {
    (2..num)
        .fold(
            BTreeSet::new(),
            |mut xs, x| if is_prime(x){ xs.insert(x); xs } else { xs }
        )
}

fn print(mut num: usize, mut factors: BTreeSet<usize>) {
    if num == 1 {
        println!("  | 1");
        return;
    }

    if let Some(x) = factors.first() {
        if num % x == 0 {
            println!("{x} | {num}");
            num /= x;
            print(num, factors)
        } else {
            factors.pop_first();
            print(num, factors);
        }
    }
}

fn main() {
    let number = std::env::args()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();

    print(number, findallprimes(number));
}
