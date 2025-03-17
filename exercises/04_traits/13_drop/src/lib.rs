// TODO: Drob bomb를 구현하세요.
// 불필요한 drop이 발생하면 패닉을 일으킵니다.
// 요구하는 API는 테스트에서 확인할 수 있습니다.

struct DropBomb {
    defused: bool,
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.defused {
            panic!();
        }
    }
}
impl DropBomb {
    pub fn new() -> Self{
        DropBomb {
            defused: false,
        }
    }

    pub fn defuse(&mut self) {
        self.defused = true;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
