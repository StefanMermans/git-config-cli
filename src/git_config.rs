use std::process::Command;

use crate::Profile;

pub fn set_local_profile(profile: &Profile) {
    let result = Command::new("git")
        .args([
            "config",
            "user.email",
            format!("{}", &profile.email).as_str(),
        ])
        .output()
        .expect("Failed to run command");

    if !result.status.success() {
        println!("Failed to set email!");
    }

    let result = Command::new("git")
        .args(["config", "user.name", format!("{}", &profile.name).as_str()])
        .output()
        .expect("Failed to run command");

    if !result.status.success() {
        println!("Failed to set name!");
    }
}
