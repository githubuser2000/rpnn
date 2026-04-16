use std::collections::BTreeSet;

pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.max(1)
}

pub fn primfaktoren(mut n: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut d = 2;
    while d * d <= n {
        while n % d == 0 {
            out.push(d);
            n /= d;
        }
        d += 1;
    }
    if n > 1 { out.push(n); }
    out
}

pub fn divisors(n: usize) -> BTreeSet<usize> {
    let mut out = BTreeSet::new();
    for i in 1..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            out.insert(i);
            out.insert(n / i);
        }
    }
    out
}
