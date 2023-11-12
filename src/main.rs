use std::env;

use crate::storage::Storage;
use action::ActionType;
use git_config::set_local_profile;
use inquire::{Confirm, MultiSelect, Select, Text, InquireError};
use profile::Profile;

mod action;
mod git_config;
mod profile;
mod storage;

fn create_credentials_actions(storage: &mut Storage) -> Result<(), InquireError> {
    let name = Text::new("Enter a name").prompt()?;
    let email = Text::new("Enter an email").prompt()?;
    let title = Text::new("What should the profile be called?")
        .prompt()?;

    let credentials = Profile { title, email, name };

    println!(
        "Creating profile\nTitle: {}\nName:  {}\nEmail: {}",
        &credentials.title, &credentials.name, &credentials.email
    );

    let confirm = Confirm::new("Is this correct? (y/n)").prompt()?;

    if !confirm {
        create_credentials_actions(storage)?;
        return Ok(());
    }

    storage.store(credentials);

    Ok(())
}

fn select_profile(storage: &Storage) -> Result<(), InquireError> {
    let profiles = storage.profiles();

    if profiles.len() == 0 {
        println!("No profiles available");

        return Ok(());
    }

    let selected = Select::new("What user do you want to use", storage.profiles())
        .prompt()?;

    set_local_profile(&selected)?;

    Ok(())
}

fn delete_profile(storage: &mut Storage) -> Result<(), InquireError> {
    let selected = MultiSelect::new("Select profiles to delete", storage.profiles())
        .prompt()?;

    for profile in selected {
        storage.delete_profile(&profile);
    }

    Ok(())
}

fn inquire(storage: &mut Storage) -> Result<(), InquireError> {
    let action = Select::new("What do you want to do?", ActionType::as_vec())
        .prompt()
        .expect("Something went wrong");

    match action {
        ActionType::SelectProfile => select_profile(storage),
        ActionType::CreateProfile => create_credentials_actions(storage),
        ActionType::DeleteProfile => delete_profile(storage),
    }
}

fn main() {
    let home = env::var("HOME").expect("Failed to get HOME dir");
    let storage_dir = format!("{}/.config/git-config-cli", home);
    let mut storage = Storage::new(storage_dir, "profiles.json".to_string());

    inquire(&mut storage).expect("Something went wrong!");
}
