#![feature(map_first_last)]

use std::{collections::BTreeSet, u128};

fn is_prime(num: u128) -> bool {
    !((2..num)
        .any(|x| num % x == 0))
}

fn findallprimes(num: u128) -> BTreeSet<u128> {
    (2..num)
        .filter(|x| is_prime(*x))
        .collect()
}

fn factorize(mut num: u128, mut factors: BTreeSet<u128>) {
    if num == 1 {
        println!("    | 1");
        return;
    }

    if let Some(x) = factors.first() {
        if num % x == 0 {
            println!("{:<3} | {num}", x );
            num /= x;
            factorize(num, factors)
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

    factorize(number, findallprimes(number));
}
