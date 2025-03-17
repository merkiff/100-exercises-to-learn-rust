// TODO: title과 description이 접근자를 통해 반환되면, 정규화되어야(normalized) 합니다.
// 예를 들어, 문자열 앞과 뒤의 공백은 제거되어야 합니다.
// 러스트에는 이를 도울 수 있는 표준 라이브러리 메서드가 있지만, String의 문서에서는 찾을 수 없죠.
// 그것이 어디에 있고 어떻게 사용하는지 찾을 수 있나요?


pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn title(&self) -> &str {
        self.title.trim()
    }

    pub fn description(&self) -> &str {
        self.description.trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let ticket = Ticket {
            title: "   A title ".to_string(),
            description: " A description   ".to_string(),
            status: "To-Do".to_string(),
        };

        assert_eq!("A title", ticket.title());
        assert_eq!("A description", ticket.description());
    }
}
