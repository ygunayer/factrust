extern crate rayon;
#[macro_use] extern crate itertools;

use std::env;
use std::collections::HashMap;

use rayon::prelude::*;

trait PrimalityChecker {
    fn is_prime(&mut self, n: i64) -> bool;
}

struct EratosthenesSieve {
    is_parallel: bool,
    limit: i64,
    sieve: Option<HashMap<i64, bool>>
}

impl EratosthenesSieve {
    fn new(limit: i64, is_parallel: bool) -> EratosthenesSieve {
        EratosthenesSieve {
            is_parallel: is_parallel,
            limit: limit,
            sieve: None
        }
    }

    fn get_sieve(&mut self) -> &HashMap<i64, bool> {
        let limit = self.limit;

        let sync = !self.is_parallel;

        return self.sieve.get_or_insert_with(move || {
            let iters: Vec<(i64, i64)> = iproduct!(2..limit, 2..limit).collect();
            if sync {
                println!("snync");
                let mut sieve: HashMap<i64, bool> = HashMap::new();
                for (a, b) in iters {
                    sieve.insert(a * b, true);
                }
                return sieve;
            }

            println!("VALLA PARALLEL");
            return iters
                .par_iter()
                .fold(|| HashMap::new(), |mut acc, (a, b)| { acc.insert(a * b, true); return acc; })
                .reduce(|| HashMap::new(), |mut a, b| { a.extend(b); return a; });
        });
    }
}

impl PrimalityChecker for EratosthenesSieve {
    fn is_prime(&mut self, n: i64) -> bool {
        return !*self.get_sieve().get(&n).unwrap_or(&false);
    }
}


trait Factorizer {
    fn factorize(&self, number: i64) -> Vec<i64>;
}

struct TrialDivision ();

impl Factorizer for TrialDivision {
    fn factorize(&self, mut number: i64) -> Vec<i64> {
        let mut factors = Vec::new();

        if number < 0 {
            number = -number;
            factors.push(-1);
        }

        if number < 2 {
            factors.push(number);
            return factors;
        }

        let mut x = 2;

        while number >= x {
            let q = number / x;
            let r = number % x;
            if r == 0 {
                factors.push(x);
                number = q;
            } else {
                x += 1;
            }
        }

        return factors;
    }
}

fn main() {
    let is_parallel: bool = env::var("PAR").ok().filter(|v| v == "1").is_some();
    let mut checker = EratosthenesSieve::new(10000, is_parallel);
    println!("{}", checker.is_prime(4));
    println!("{}", checker.is_prime(17));

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => println!("Please specify the number to factor"),
        n if n >= 2 => {
            match args[1].parse::<i64>() {
                Ok(x) => {
                    let factorizer = TrialDivision();
                    let factors = factorizer.factorize(x);
                    println!("{:?}", factors);
                },
                Err(e) => println!("{}: {}", e, args[1])
            }
        },
        _ => println!("nani")
    }
}
