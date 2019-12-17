fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..n {
        if n % i ==0 {
            return false;
        }
    }

    return true;
}

fn main() {
    let max: u32 = 100_000;
    let mut primes = vec![2];

    for x in 3..max {
        if is_prime(x) {
            primes.push(x);
        }
    }

    println!("Primes less than {:?}:", max);
    println!("{:?}", primes);
}

