pub fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
