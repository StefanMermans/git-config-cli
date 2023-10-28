use std::fs::{self, File};

use crate::profile::Profile;

pub struct Storage {
    storage_dir: String,
}

impl Storage {
    pub fn new() -> Self{
        Self {
            // TODO: Dynamic storage dir
            storage_dir: "./data/profiles.json".to_string(),
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
        let json_string = match fs::read_to_string(&self.storage_dir) {
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
        
        // TODO: write to file
        File::metadata(&self.storage_dir).is_ok()
    }
}
