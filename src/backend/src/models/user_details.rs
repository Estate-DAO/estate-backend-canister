use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

// UserDetails scope
#[derive(CandidType,Serialize,Deserialize,Default, Clone, Debug)]
pub struct UserDetails {
    pub adults: Vec<AdultDetail>,
    pub children: Vec<ChildDetail>,
}

impl UserDetails {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_adult(&mut self, adult: AdultDetail) {
        self.adults.push(adult);
    }

    pub fn add_child(&mut self, child: ChildDetail) -> Result<(), String> {
        if child.age > 18 {
            return Err("Child must be under 18 years old".into());
        }
        self.children.push(child);
        Ok(())
    }

    pub fn get_primary_contact(&self) -> Option<(String, String)> {
        self.adults.first().and_then(|adult| {
            match (&adult.email, &adult.phone) {
                (Some(email), Some(phone)) => Some((email.clone(), phone.clone())),
                _ => None
            }
        })
    }

    pub fn total_guests(&self) -> usize {
        self.adults.len() + self.children.len()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.adults.is_empty() {
            return Err("At least one adult required".into());
        }

        // Validate primary adult has contact info
        if let Some(adult) = self.adults.first() {
            if adult.email.is_none() || adult.phone.is_none() {
                return Err("Primary adult must provide email and phone".into());
            }
        }

        // todo: validate that all the ages of children are < 18

        Ok(())
    }
}

#[derive(CandidType,Serialize,Deserialize,Default, Clone, Debug)]
pub struct AdultDetail {
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>, // Only for first adult
    pub phone: Option<String>, // Only for first adult
}

#[derive(CandidType,Serialize,Deserialize,Default, Clone, Debug)]
pub struct ChildDetail {
    pub first_name: String,
    pub last_name: Option<String>,
    pub age: u8,
}
