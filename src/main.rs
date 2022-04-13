#![feature(map_first_last)]

use std::collections::BTreeSet;

fn is_prime(num: usize) -> bool {
    for x in 1..num {
        if num % x == 0 {
            return true;
        }
    }
    false
}

fn findallprimes(num: usize) -> Vec<usize> {
    let mut rt = Vec::new();
    (1..num).for_each(|x| if is_prime(x){ rt.push(x); });
    rt
}

fn factorize(num: usize) -> BTreeSet<usize> {
    let mut rt = BTreeSet::new();
    findallprimes(num)
        .into_iter()
        .for_each(|x| if num % x == 0 { rt.insert(x); });
    rt
}

fn print(mut num: usize, mut factors: BTreeSet<usize>) {
    
    if num == 1 {
        println!("  | 1");
        return;
    }

    if let Some(x) = factors.pop_first() {
        if num % x == 0 {
            println!("{x}|{num}");
            num /= x;
            print(num, factors.pop_first())
        } else {
            print(num, factors);
        }
    }
}

fn main() {
    let number = 45470971;
    print(number, factorize(number));
}
