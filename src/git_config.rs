use std::{process::Command, io::Error};

use crate::Profile;

pub fn set_local_profile(profile: &Profile) -> Result<(), Error> {
    let result = Command::new("git")
        .args([
            "config",
            "user.email",
            format!("{}", &profile.email).as_str(),
        ])
        .output()?;

    if !result.status.success() {
        println!("Failed to set email!");
    }

    let result = Command::new("git")
        .args(["config", "user.name", format!("{}", &profile.name).as_str()])
        .output()?;

    if !result.status.success() {
        println!("Failed to set name");
    }

    Ok(())
}
