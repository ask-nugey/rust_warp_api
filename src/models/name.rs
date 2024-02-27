use std::str::FromStr;
use warp::{reject, Rejection};

#[derive(Clone, Debug)]
pub struct Name(String);

impl Name {
    pub fn new(name: &str) -> Result<Self, NameError> {
        let size = name.chars().count();
        if size < 1 || size > 10 {
            return Err(NameError::InvalidLength);
        }

        if name.chars().any(|c| !c.is_ascii_alphabetic()) {
            return Err(NameError::InvalidCharacter);
        }

        Ok(Name(name.to_string()))
    }
}

impl FromStr for Name {
    type Err = NameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Name::new(s)
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub enum NameError {
    InvalidLength,
    InvalidCharacter,
}

impl warp::reject::Reject for NameError {}

pub async fn validate_name(name: String) -> Result<Name, Rejection> {
    Name::new(&name).map_err(reject::custom)
}

#[warn(dead_code)]
impl NameError {
    pub fn message(&self) -> &'static str {
        match self {
            NameError::InvalidLength => "名前は10文字以内です",
            NameError::InvalidCharacter => "名前が使用できる文字種はA-Z, a-zです",
        }
    }
}
