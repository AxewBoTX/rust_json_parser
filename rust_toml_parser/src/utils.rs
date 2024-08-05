// ----- `IteratorList` -----
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IteratorList<ElementType> {
    list: Vec<ElementType>,
    max_cursor_value: usize,
    cursor: usize,
}
impl<ElementType> IteratorList<ElementType>
where
    ElementType: std::fmt::Debug + Clone + Eq + PartialEq,
{
    pub fn new(list: &Vec<ElementType>) -> IteratorList<ElementType> {
        let max_cursor_value = list.len() - 1;
        return IteratorList {
            list: list.to_vec(),
            cursor: 0,
            max_cursor_value,
        };
    }
    // returns the current element and move the cursor to the next element
    pub fn current(&mut self) -> Option<ElementType> {
        if self.cursor > self.max_cursor_value {
            return None;
        } else {
            self.cursor += 1;
            return Some(self.list[self.cursor - 1].clone());
        }
    }
    // returns the current element w/o moving the cursor to the next element
    pub fn peek(&self) -> Option<ElementType> {
        if self.cursor > self.max_cursor_value {
            return None;
        } else {
            return Some(self.list[self.cursor].clone());
        }
    }
    // returns the nth element after cursor w/o advancing the cursor
    pub fn peek_after(&self, n: usize) -> Option<ElementType> {
        if self.cursor + n > self.max_cursor_value {
            return None;
        } else {
            return Some(self.list[self.cursor + n].clone());
        }
    }
    // returns the nth element before cursor w/o advancing the cursor
    pub fn peek_before(&self, n: usize) -> Option<ElementType> {
        if (self.cursor as isize) - (n as isize) < 0 {
            return None;
        } else {
            return Some(self.list[self.cursor - n].clone());
        }
    }
    // get the index of an element by it's value
    pub fn index(&self, element: ElementType) -> Option<usize> {
        let mut parser = IteratorList::new(&self.list.clone());
        while let Some(value) = parser.current() {
            if value == element {
                return Some(parser.cursor - 1);
            }
        }
        return None;
    }
}

pub fn is_integer(input: String) -> bool {
    return input.parse::<i64>().is_ok();
}
pub fn is_float(input: String) -> bool {
    return input.parse::<f64>().is_ok();
}
