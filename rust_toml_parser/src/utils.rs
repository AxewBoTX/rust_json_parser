pub fn is_integer(input: String) -> bool {
    return input.parse::<i64>().is_ok();
}
pub fn is_float(input: String) -> bool {
    return input.parse::<f64>().is_ok();
}
