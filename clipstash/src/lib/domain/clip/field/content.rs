use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize , Serialize)]
pub struct Content(String);

impl Content {
    pub fn new(content: &str) -> Result<Self, ClipError> {
        if !content.trim().is_empty() {
            Ok(Self(content.to_owned()))
        } else {
            Err(ClipError::EmptyContent)
        }
    }

    // This is used to convert the Content(String) to a inner string
    pub into_inner(self) -> String {
        self.0
    }

    pub as_str(&self) -> &str{
        self.0.as_str()
    }
    
}
