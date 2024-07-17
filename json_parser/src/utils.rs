pub fn is_number(input: &str) -> bool {
    return input.parse::<f64>().is_ok();
}
