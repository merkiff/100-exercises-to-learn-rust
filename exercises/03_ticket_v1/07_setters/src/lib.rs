// TODO: Ticket 구조체의 각 필드에 대여-가변 설정자(&mut-setter)를 추가하세요.
//   `Ticket::new`와 동일한 조건을 준수하세요.
//   동일한 로직을 추출해 재사용하면 더 좋습니다.
//   비공개(private) 함수나 비공개 정적(static) 메서드를 사용할 수 있습니다.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        //조건 검사 로직은 필드 별로 분리
        check_title(&title);
        check_description(&description);
        check_status(&status);

        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    //setter 생성
    pub fn set_title(&mut self, title: String) {
        check_title(&title);
        self.title = title
    }
    pub fn set_description(&mut self, description: String) {
        check_description(&description);
        self.description = description
    }
    pub fn set_status(&mut self, status: String) {
        check_status(&status);
        self.status = status
    }
}

fn check_title(title: &String) {
    if title.is_empty() {
        panic!("Title cannot be empty");
    }
    if title.len() > 50 {
        panic!("Title cannot be longer than 50 bytes");
    }
}

fn check_description(description: &String) {
    if description.is_empty() {
        panic!("Description cannot be empty");
    }
    if description.len() > 500 {
        panic!("Description cannot be longer than 500 bytes");
    }
}
fn check_status(status: &String) {
    if status != "To-Do" && status != "In Progress" && status != "Done" {
        panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn works() {
        let mut ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        ticket.set_title("A new title".into());
        ticket.set_description("A new description".into());
        ticket.set_status("Done".into());

        assert_eq!(ticket.title(), "A new title");
        assert_eq!(ticket.description(), "A new description");
        assert_eq!(ticket.status(), "Done");
    }

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_title("".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_description("".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_title(overly_long_title())
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_description(overly_long_description())
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_status("Funny".into());
    }
}
