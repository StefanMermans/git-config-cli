use std::fs::{self};

use crate::profile::Profile;

pub struct Storage {
    file: String,
    dir: String,
}

impl Storage {
    pub fn new() -> Self{
        Self {
            // TODO: Dynamic storage dir
            dir: "./data".to_string(),
            file: "profiles.json".to_string(),
        }   
    }

    pub fn delete_profile(&mut self, profile_to_delete: &Profile) {
        let mut profiles = self.load_profiles();
        
        if let Some(index) = profiles.iter().position(|profile| profile.title == profile_to_delete.title) {
            profiles.swap_remove(index);
        }

        self.save_profiles(&profiles);
    }

    pub fn profiles(&self) -> Vec<Profile>  {
        self.load_profiles()
    }

    pub fn store(&mut self, credentials: Profile) {
        let mut profiles = self.load_profiles();
        profiles.push(credentials);
        self.save_profiles(&profiles);
    }

    fn load_profiles(&self) -> Vec<Profile> {
        let json_string = match fs::read_to_string(&self.full_path()) {
            Ok(data) => data,
            Err(_) => return vec![],
        };

        match serde_json::from_str(&json_string) {
            Ok(data) => data,
            Err(_) => vec![],
        }
    }

    fn save_profiles(&self, profiles: &Vec<Profile>) {
        let json = match serde_json::to_string(profiles) {
            Ok(json) => json,
            Err(_) => "[]".to_string(),
        };
        
        println!("Saving: {}", &json);

        fs::create_dir_all(&self.dir).expect("Failed to create all dirs");
        fs::write(&self.full_path(), json).expect("Failed to save file");
    }

    fn full_path(&self) -> String {
        return format!("{}/{}", &self.dir, &self.file)
    }
}
