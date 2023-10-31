use std::env;

use crate::storage::Storage;
use action::ActionType;
use git_config::set_local_profile;
use inquire::{Confirm, MultiSelect, Select, Text};
use profile::Profile;

mod action;
mod git_config;
mod profile;
mod storage;

fn create_credentials_actions(storage: &mut Storage) {
    let name = Text::new("Enter a name").prompt().unwrap();
    let email = Text::new("Enter an email").prompt().unwrap();
    let title = Text::new("What should the profile be called?")
        .prompt()
        .unwrap();

    let credentials = Profile { title, email, name };

    println!(
        "Creating profile\nTitle: {}\nName:  {}\nEmail: {}",
        &credentials.title, &credentials.name, &credentials.email
    );

    let confirm = Confirm::new("Is this correct? (y/n)").prompt().unwrap();

    if !confirm {
        create_credentials_actions(storage);
        return;
    }

    storage.store(credentials);
}

fn select_profile(storage: &Storage) {
    let profiles = storage.profiles();

    if profiles.len() == 0 {
        println!("No profiles available");

        return;
    }

    let selected = Select::new("What user do you want to use", storage.profiles())
        .prompt()
        .unwrap();

    set_local_profile(&selected);
}

fn delete_profile(storage: &mut Storage) {
    let selected = MultiSelect::new("Select profiles to delete", storage.profiles())
        .prompt()
        .unwrap();

    for profile in selected {
        storage.delete_profile(&profile);
    }
}

fn main() {
    let home = env::var("HOME").unwrap();
    let storage_dir = format!("{}/.config/git-config-cli", home);
    let mut storage = Storage::new(storage_dir, "profiles.json".to_string());

    let action = Select::new("What do you want to do?", ActionType::as_vec())
        .prompt()
        .unwrap();

    match action {
        ActionType::SelectProfile => select_profile(&storage),
        ActionType::CreateProfile => create_credentials_actions(&mut storage),
        ActionType::DeleteProfile => delete_profile(&mut storage),
    }
}
