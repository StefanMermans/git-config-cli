use action::ActionType;
use inquire::{Select, Text, Confirm};
use profile::Profile;
use crate::storage::Storage;

mod profile;
mod action;
mod storage;
mod git_config;

fn create_credentials_actions(storage: &mut Storage) {
    let name = Text::new("Enter a name").prompt().unwrap();
    let email = Text::new("Enter an email").prompt().unwrap();
    let title = Text::new("What should the profile be called?").prompt().unwrap();

    let credentials = Profile {
        title,
        email,
        name,
    };

    println!("Creating profile\nTitle: {}\nName:  {}\nEmail: {}", &credentials.title, &credentials.name, &credentials.email); 

    let confirm = Confirm::new("Is this correct? (y/n)")
        .prompt()
        .unwrap();

    if !confirm {
        create_credentials_actions(storage);
        return;
    }

    storage.store(credentials);
}

fn select_profile(storage: &Storage) {
    let selected = Select::new("What user do you want to use", storage.credentials_vec())
        .prompt()
        .unwrap();

    println!("Will use email {}", selected.email);
}

fn main() {
    let mut storage = Storage::new();

    let action = Select::new("What do you want to do?", ActionType::as_vec()) 
        .prompt()
        .unwrap();

    match action {
        ActionType::SelectProfile => select_profile(&storage),
        ActionType::CreateProfile => create_credentials_actions(&mut storage),
    }
}
