#![no_std]
#![forbid(unsafe_code)]

pub fn add(left: i32, right: i32) -> Option<i32> {
    left.checked_add(right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_integers() {
        assert_eq!(add(2, 2), Some(4));
    }

    #[test]
    fn detects_integer_overflow() {
        assert_eq!(add(i32::MAX, 1), None);
    }

    #[test]
    fn detects_integer_underflow() {
        assert_eq!(add(i32::MIN, -1), None);
    }
}
