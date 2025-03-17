pub mod ticket {
    pub struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {
        pub fn new(title: String, description: String, status: String) -> Ticket {
            if title.is_empty() {
                panic!("Title cannot be empty");
            }
            if title.len() > 50 {
                panic!("Title cannot be longer than 50 bytes");
            }
            if description.is_empty() {
                panic!("Description cannot be empty");
            }
            if description.len() > 500 {
                panic!("Description cannot be longer than 500 bytes");
            }
            if status != "To-Do" && status != "In Progress" && status != "Done" {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
            }

            Ticket {
                title,
                description,
                status,
            }
        }
    }
}

// TODO: **예외적으로**, 이 연습에서는 ticket과 test 모듈을 모두 수정할 수 있습니다.
#[cfg(test)]
mod tests {
    // TODO: 아래의 구문에 발생하는 컴파일러 오류를 제거하기 위해 `pub` modifier를 부모 모듈에 추가하세요.
    use super::ticket::Ticket;

    // 조심하세요! use statement를 컴파일할 수 있도록 가시성(visibility)을 바꾼 다음
    // 이 함수가 컴파일되는 건 원하지 않습니다!
    // 실제로 컴파일되지 않는지 확인했다면 주석 처리하세요.
    fn should_not_be_possible() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());

        // 실행하려 하면 다음과 같은 오류가 발생할 것입니다:
        //
        // error[E0616]: field `description` of struct `Ticket` is private
        //    |
        //    |              assert_eq!(ticket.description, "A description");
        //    |                         ^^^^^^^^^^^^^^^^^^
        //
        // TODO: 실제로 컴파일되지 않는지 확인했다면, 다음 연습으로 나아가기 위해 아래 줄을 주석처리하세요!
        //assert_eq!(ticket.description, "A description");
    }

    fn encapsulation_cannot_be_violated() {
        //이 경우에도 위에서 발생한 것과 비슷한 오류가 발생합니다.
        // (이전 테스트에서 잘못된 줄에 주석을 달아야만 다음 컴파일 단계에서 컴파일 오류가 발생합니다!).
        // 이제 `Ticket::new`가 `Ticket` 인스턴스를 얻는 유일한 방법임을 증명합니다.
        // 잘못된 제목이나 설명으로 티켓을 생성하는 것은 불가능합니다!
        // TODO: 실제로 컴파일되지 않는지 확인했다면, 다음 연습으로 나아가기 위해 아래 줄을 주석처리하세요!
        // let ticket = Ticket {
        //     title: "A title".into(),
        //     description: "A description".into(),
        //     status: "To-Do".into(),
        // };
    }
}
