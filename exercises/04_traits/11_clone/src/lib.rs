// TODO: 필요한 Clone 구현을 추가하여 코드가 컴파일될 수 있도록 하세요.
pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    (ticket.clone(), ticket.summary())
}

#[derive(Clone)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}
