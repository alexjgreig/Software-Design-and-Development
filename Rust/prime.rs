fn is_prime(n: u32) -> bool {
    let limit = (n as f64).sqrt() as u32;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut count = 0;
    let mut num = 2;
    while count < 1000 {
        if is_prime(num) {
            count += 1;
            println!("Prime number {} is {}", count, num);
        }
        num += 1;
    }
}
