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
    fn u16_ref_size() {
        assert_eq!(size_of::<&u16>(), 8);
    }

    #[test]
    fn u64_mut_ref_size() {
        assert_eq!(size_of::<&mut u64>(), 8);
    }

    #[test]
    fn ticket_ref_size() {
        assert_eq!(size_of::<&Ticket>(), 8);
    }
}
