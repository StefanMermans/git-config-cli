use std::fmt::Display;

pub enum ActionType {
    CreateProfile,
    SelectProfile,
}

impl ActionType {
    pub fn label(&self) -> String {
        match self {
            ActionType::CreateProfile => "Create profile".to_string(),
            ActionType::SelectProfile => "Select profile".to_string(),
         }
    }
    
    pub fn as_vec() -> Vec<ActionType> {
        // This is kinda annoying when adding new actions
        return vec![
            ActionType::CreateProfile,
            ActionType::SelectProfile,
        ];
    }
}

impl Display for ActionType {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

