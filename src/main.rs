#![feature(map_first_last)]

use std::{collections::BTreeSet, u64};

fn is_prime(num: u64) -> bool {
    !((2..num)
        .any(|x| num % x == 0))
}

fn findallprimes(num: u64) -> BTreeSet<u64> {
    (2..f64::sqrt(num as f64).floor() as u64)
        .filter(|x| is_prime(*x))
        .collect()
}

fn factorize(mut num: u64, mut factors: BTreeSet<u64>) {
    if num == 1 {
        println!("{:<3} | 1", "");
        return;
    }

    if let Some(x) = factors.first() {
        if num % x == 0 {
            println!("{:<3} | {num}", x );
            num /= x;
            factorize(num, factors);
        } else {
            factors.pop_first();
            factorize(num, factors);
        }
    }
}

fn main() {
    let number = std::env::args()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();

    if is_prime(number) {
        println!("{number} is a prime number");
        return;
    }

    factorize(number, findallprimes(number));
}
