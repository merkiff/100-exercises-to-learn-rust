// TODO: 테스트를 컴파일하고 통과할 수있게 필요한 트레이트를 구현하세요.
// 테스트를 수정해서는 안 됩니다.

use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl Add for WrappingU32 {
    type Output = Self;
         fn add(self, other: WrappingU32) -> Self {
            Self{
                value: self.value.wrapping_add(other.value)
            }
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
