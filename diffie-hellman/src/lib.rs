pub fn private_key(p: u64) -> u64 {
    p - 1
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    module_exponential(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    module_exponential(b_pub, a, p)
}

fn module_exponential(base: u64, exp: u64, module: u64) -> u64 {
    let mut power = 1;
    for _ in 0..exp {
        power = (power * base) % module
    }

    power
}

