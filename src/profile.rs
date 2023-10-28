use std::fmt::Display;

#[derive(Clone)]
pub struct Profile {
    pub title: String,
    pub name: String,
    pub email: String,
}

impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

