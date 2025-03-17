pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: `todo!()`를 올바른 **스택 크기**로 수정하세요.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 24);
    }

    #[test]
    fn ticket_size() {
        // 꽤 까다로운 질문입니다!
        // "직관적"인 답이 이번에는 맞겠지만,
        // 일반적으로 구조체의 메모리 레이아웃은 더 복잡한 주제입니다.
        // 만약 궁금하다면, Rust Reference의 "Type layout:" 섹션을 확인해보세요.
        // https://doc.rust-lang.org/reference/type-layout.html
        assert_eq!(size_of::<Ticket>(), 72);
    }
}
