// TODO: `todo!()`를 올바른 스택 크기로 수정하세요.
#[cfg(test)]
mod tests {
    use std::mem::size_of;

    #[test]
    fn u16_size() {
        assert_eq!(size_of::<u16>(), 2);
    }

    #[test]
    fn i32_size() {
        assert_eq!(size_of::<i32>(), 4);
    }

    #[test]
    fn bool_size() {
        assert_eq!(size_of::<bool>(), 1);
    }
}
