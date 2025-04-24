use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Permission {
    pub name: String,
    pub protection_level: String,
    pub description: String,
    pub group: String,
    pub is_dangerous: bool,
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}\nProtection Level: {}\nDescription: {}\nGroup: {}\nDangerous: {}",
            self.name,
            self.protection_level,
            self.description,
            self.group,
            self.is_dangerous
        )
    }
}

impl Default for Permission {
    fn default() -> Self {
        Self {
            name: String::new(),
            protection_level: String::new(),
            description: String::new(),
            group: String::new(),
            is_dangerous: false,
        }
    }
} 