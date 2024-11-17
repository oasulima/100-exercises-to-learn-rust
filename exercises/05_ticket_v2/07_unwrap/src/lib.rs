// TODO: `easy_ticket` should panic when the title is invalid.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".
const TITLE_CANNOT_BE_EMPTY: &str = "Title cannot be empty";
const TITLE_CANNOT_BE_LONGER_THAN_50_BYTES: &str = "Title cannot be longer than 50 bytes";
const DESCRIPTION_CANNOT_BE_EMPTY: &str = "Description cannot be empty";
const DESCRIPTION_CANNOT_BE_LONGER_THAN_500_BYTES: &str =
    "Description cannot be longer than 500 bytes";

fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    let ticket = Ticket::new(title.clone(), description, status.clone());
    match ticket {
        Ok(ticket) => ticket,
        Err(error) => match error {
            DESCRIPTION_CANNOT_BE_EMPTY | DESCRIPTION_CANNOT_BE_LONGER_THAN_500_BYTES => {
                Ticket::new(title, "Description not provided".to_string(), status).unwrap()
            }
            _ => {
                panic!("{error}")
            }
        },
    }
}

#[derive(Debug, PartialEq, Clone)]
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
    pub fn new(title: String, description: String, status: Status) -> Result<Ticket, &'static str> {
        if title.is_empty() {
            return Err(TITLE_CANNOT_BE_EMPTY);
        }
        if title.len() > 50 {
            return Err(TITLE_CANNOT_BE_LONGER_THAN_50_BYTES);
        }
        if description.is_empty() {
            return Err(DESCRIPTION_CANNOT_BE_EMPTY);
        }
        if description.len() > 500 {
            return Err(DESCRIPTION_CANNOT_BE_LONGER_THAN_500_BYTES);
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
