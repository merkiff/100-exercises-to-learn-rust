// TODO: title error와 description error에 대응하는 두 개의 배리언트를 사용합니다.
// 각각의 배리언트는 무엇이 잘못되었는지 설명하는 문자열을 포함해야 합니다.
// `Ticket::new`의 구현 또한 업데이트해야 할 것입니다.
#[derive(Debug)]
enum TicketNewError {
    TitleError(String),
    DescriptionError(String),
}


// TODO: `easy_ticket`은 title이 잘못되면 panic을 일으켜야 하고,
// `TicketNewError` 열거형에서 관련된 배리언트에 저장된 에러 메시지를 사용합니다.
// description이 잘못되면 "Description not provided"라는 기본 description을 사용합니다.
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(), description, status.clone()) {
        Ok(ticket) => ticket,
        Err(TicketNewError::TitleError(err)) => {
            panic!("{}", err);
        }
        Err(TicketNewError::DescriptionError(_)) => {
            Ticket::new(title, "Description not provided".into(), status).unwrap()
        }

    }
}

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleError("Title cannot be empty".to_string()));
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleError("Title cannot be longer than 50 bytes".to_string()));
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionError("Description cannot be empty".to_string()));
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionError("Description cannot be longer than 500 bytes".to_string()));
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
}
