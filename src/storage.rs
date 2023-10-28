use crate::profile::Profile;

pub struct Storage {
    credentials_vec: Vec<Profile>,
}

impl Storage {
    pub fn new() -> Self{
        // TODO: Not just static data (Get from file)
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

    pub fn delete_profile(&mut self, profile_to_delete: &Profile) {
        // TODO: remove from a file.
        if let Some(index) = self.credentials_vec.iter().position(|profile| profile.title == profile_to_delete.title) {
            self.credentials_vec.swap_remove(index);
        }
    }

    pub fn profiles(&self) -> Vec<Profile>  {
        (*self.credentials_vec).to_vec()
    }

    pub fn store(&mut self, credentials: Profile) {
        // TODO: Store in a file
        self.credentials_vec.push(credentials);
    }
}
