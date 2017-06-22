pub fn primes_up_to(n: usize) -> Vec<usize> {
    let mut sieve: Vec<bool> = vec![true; (n + 1)];

    sieve[0] = false;
    sieve[1] = false;

    for i in 2..(n + 1) {
        if !sieve[i] {
            continue;
        }

        let mut j = i * 2;
        while j <= n {
            sieve[j] = false;
            j += i
        }
    }

    let mut primes = Vec::new();
    for i in 2..(n + 1) {
        if sieve[i] {
            primes.push(i);
        }
    }

    primes
}
