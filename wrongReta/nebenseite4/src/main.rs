use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Verwendung: {} <positive ganze Zahl>", args[0]);
        std::process::exit(1);
    }

    let n: u64 = match args[1].parse() {
        Ok(v) if v >= 2 => v,
        _ => {
            eprintln!("Fehler: Bitte eine ganze Zahl >= 2 angeben.");
            std::process::exit(1);
        }
    };

    let primfaktoren = prime_factors(n);

    println!();
    //println!("Primfaktoren:");
    print_factor_list(&primfaktoren);

    println!();
    //println!("Zerlegungen in 2 Faktoren:");
    let pairs = factor_groupings(n, 2);
    if pairs.is_empty() {
        println!("  Keine.");
    } else {
        for p in &pairs {
            print!("{}, ", join_factors(p));
        }
    }

    println!();
    //println!("Zerlegungen in 3 Faktoren:");
    let triples = factor_groupings(n, 3);
    if triples.is_empty() {
        println!("  Keine.");
    } else {
        for t in &triples {
            print!("{}, ", join_factors(t));
        }
    }
    println!();
}

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();

    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    let mut d = 3;
    while d * d <= n {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
        d += 2;
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}

fn print_factor_list(factors: &[u64]) {
    if factors.is_empty() {
        print!("  Keine Primfaktoren.");
        return;
    }

    let text = factors
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<_>>()
        .join("*");

    print!("{}", text);
}

fn join_factors(factors: &[u64]) -> String {
    factors
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("*")
}

fn factor_groupings(n: u64, parts: usize) -> Vec<Vec<u64>> {
    let mut result = Vec::new();
    collect_groupings(n, parts, 2, &mut Vec::new(), &mut result);
    result
}

fn collect_groupings(
    remaining: u64,
    parts_left: usize,
    min_factor: u64,
    current: &mut Vec<u64>,
    result: &mut Vec<Vec<u64>>,
) {
    if parts_left == 1 {
        if remaining >= min_factor {
            current.push(remaining);
            result.push(current.clone());
            current.pop();
        }
        return;
    }

    let max_factor = integer_nth_root_floor(remaining, parts_left as u32);

    for f in min_factor..=max_factor {
        if remaining % f == 0 {
            current.push(f);
            collect_groupings(remaining / f, parts_left - 1, f, current, result);
            current.pop();
        }
    }
}

fn integer_nth_root_floor(n: u64, k: u32) -> u64 {
    if k == 1 {
        return n;
    }

    let mut x = 1;
    while pow_u64(x + 1, k) <= n {
        x += 1;
    }
    x
}

fn pow_u64(mut base: u64, mut exp: u32) -> u64 {
    let mut result = 1u64;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result.saturating_mul(base);
        }
        exp /= 2;
        if exp > 0 {
            base = base.saturating_mul(base);
        }
    }
    result
}
