use std::fmt;
#[derive(Debug)]
pub enum Event {
    Products,
    Category,
    User,
}

#[derive(Debug)]
pub enum ActionType {
    INSERT,
    UPDATE,
    DELETE,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
