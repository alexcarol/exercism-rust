pub fn sum_of_multiples(n: i64, divisors: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for i in 1..n {
        for divisor in divisors {
            if i % divisor == 0 {
                sum += i;
                break;
            }
        }
    }

    sum
}
