#![feature(map_first_last)]

use std::collections::BTreeSet;

fn is_prime(num: usize) -> bool {
    for x in 2..(num-1) {
        if num % x == 0 {
            return false;
        }
    }
    true
}

fn findallprimes(num: usize) -> Vec<usize> {
    (2..num)
        .fold(
            Vec::new(),
            |mut xs, x| if is_prime(x){ xs.push(x); xs } else { xs }
        )
}

fn factorize(num: usize) -> BTreeSet<usize> {
    findallprimes(num)
        .into_iter()
        .fold(
            BTreeSet::new(),
            |mut xs, x| if num % x == 0 { xs.insert(x); xs } else { xs }
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

    print(number, factorize(number));
}
