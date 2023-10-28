use crate::profile::Profile;

pub struct Storage {
    credentials_vec: Vec<Profile>,
}

impl Storage {
    pub fn new() -> Self{
        Self {
            credentials_vec: vec![
                Profile {
                    title: "Personal".to_string(),
                    name: "Stefan Mermans".to_string(),
                    email: "stefamermans99@gmail.com".to_string(),
                },
                Profile {
                    title: "Work".to_string(),
                    name: "Stefan Mermans".to_string(),
                    email: "stefan@scrumble.nl".to_string(),
                }
            ]
        }
    }

    pub fn credentials_vec(&self) -> Vec<Profile>  {
        (*self.credentials_vec).to_vec()
    }

    pub fn store(&mut self, credentials: Profile) {
        self.credentials_vec.push(credentials);
    }
}
