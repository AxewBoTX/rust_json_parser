pub fn is_number(input: &str) -> bool {
    return input.parse::<f64>().is_ok();
}
pub fn is_bool_true(input: &str) -> bool {
    return input == "true";
}
pub fn is_bool_false(input: &str) -> bool {
    return input == "false";
}
pub fn is_null(input: &str) -> bool {
    return input == "null";
}
