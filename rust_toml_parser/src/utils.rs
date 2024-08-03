pub fn is_integer(input: String) -> bool {
    return input.parse::<i64>().is_ok();
}
pub fn is_float(input: String) -> bool {
    return input.parse::<f64>().is_ok();
}

pub trait TomlIterator<ElementType, OwnerType>
where
    ElementType: std::fmt::Debug + Eq + PartialEq + Clone,
    OwnerType: std::fmt::Debug + Eq + PartialEq + Clone,
{
    fn next(&mut self) -> Option<ElementType>;
    fn peek(&self) -> Option<ElementType>;
    fn peek_after(&self, n: usize) -> Option<ElementType>;
    fn peek_before(&self, n: usize) -> Option<ElementType>;
    fn index(&self, element: ElementType) -> Option<usize>;
}
