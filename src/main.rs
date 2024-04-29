fn main() {
    println!("RSA program");
    let number: u64 = 751;
    let p = calculate_p(number).expect("P doesn't exist for given N");
    let q = calculate_q(number);
    println!("number = {}\np = {}\nq = {}", number, p, q);
    let n = p * q;
    println!("n = p * q = {} * {} = {}", p, q, n);
    let m = (p - 1) * (q - 1);
    println!("m = (p - 1) * (q - 1) = {} * {} = {}", p - 1, q - 1, m);
    let e = calculate_e(m);
    println!(
        "The smallest e = {} (must be prime number and relatively prime with m)",
        e
    );
    let d = calculate_d(e, m);
    println!("d*e mod m = 1 and d < m; d*{} mod {} = 1; d = {}", e, m, d)
}

// calculate the previous prime
fn calculate_p(n: u64) -> Option<u64> {
    if n == 3 {
        return Some(2);
    }
    // there is no p for n = 2 or 1 or 0
    if n <= 2 {
        return None;
    }
    // as we know all primes (other than 2) are always odd so we only go over odd numbers
    for i in (2..n).filter(|i| i % 2 != 0).rev() {
        if is_prime(i) {
            return Some(i);
        }
    }
    None
}

// calculate the next prime
fn calculate_q(n: u64) -> u64 {
    if n == 0 {
        // 1 is not a prime number
        return 2;
    }
    if n == 1 {
        return 2;
    }
    if n == 2 {
        return 3;
    }
    let mut n = n;
    if n % 2 == 0 {
        // if n is even add one to get the next odd number
        n += 1
    } else {
        // if n is odd add two to get the next odd number
        n += 2
    }
    // as we know all primes (other than 2) are always odd so we only go over odd numbers
    loop {
        if is_prime(n) {
            // if n is prime return n
            return n;
        } else {
            // else go to the next odd number
            n += 2
        }
    }
}

fn is_prime(n: u64) -> bool {
    if n == 0 {
        return false;
    };
    if n == 1 {
        return false;
    };
    // check if n divides cleanly by any odd number between 2 and n-1
    // (if it divides by an even number that means it would've divided by 2)
    for i in (2..(n - 1)).filter(|i| i % 2 != 0) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn calculate_e(m: u64) -> u64 {
    let mut e: u64 = 2;
    loop {
        if gcd(m, e) == 1 {
            return e;
        } else {
            e += 1;
        }
    }
}

// simple recursive algorithm to find the GCD of two number recursively
fn gcd(number: u64, other_number: u64) -> u64 {
    if other_number == 0 {
        number
    } else {
        gcd(other_number, number % other_number)
    }
}

fn calculate_d(e: u64, m: u64) -> u64 {
    for i in 1..e + 1 {
        if (i * m + 1) % e == 0 {
            return (i * m + 1) / e;
        }
    }
    0
}
