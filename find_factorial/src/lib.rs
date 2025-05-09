pub fn factorial(num: u64) -> u64 {
    let mut count = 1;
    for i in 1..num+1{
        count *= i;
    }
    return count
}