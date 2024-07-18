pub fn is_f64(input: &str) -> bool {
    return input.parse::<f64>().is_ok();
}
pub fn is_i64(input: &str) -> bool {
    return input.parse::<i64>().is_ok();
}
