// TODO: self의 n제곱을 반환하는 power 메서드를 가진 Power 트레이트를 정의하세요.
// 트레이트 정의와 구현은 테스트를 컴파일하고 통과할 수 있어야 합니다.

// 추천: 모든 케이스를 다루려고 제네릭 구현을 시도하겠지만, 이는 꽤 복잡하고,
// num-traits와 같은 크레이트를 필요로 합니다.
// 그러니 고도의 제네릭 구현을 피하려면 간단한 매크로르 사용하는 편이 좋습니다.
// "Little book of Rust macros"(https://veykril.github.io/tlborm/)를 참고하세요.
// 하지만 꼭 그래야 할 필요는 없습니다. 세 개의 분리된 구현을 작성해도 괜찮습니다.
// 당신이 궁금하다면 더 많은 걸 탐구해보세요.

pub trait Power<RHS = Self> {
    type Output;
    fn power(self, rhs:RHS) -> Self::Output;
}

impl Power<u16> for u32 {
    type Output = u32;
    fn power(self, rhs:u16) -> Self::Output {
        let mut res = 1;
        for _ in 0..rhs {
            res *= self;
        }
        res
    }
}

impl Power<u32> for u32 {
    type Output = u32;
    fn power(self, rhs:u32) -> Self::Output {
        let mut res = 1;
        for _ in 0..rhs {
            res *= self;
        }
        res
    }
}

impl Power<&u32> for u32 {
    type Output = u32;
    fn power(self, rhs:&u32) -> Self::Output {
        let mut res = 1;
        for _ in 0..*rhs {
            res *= self;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
