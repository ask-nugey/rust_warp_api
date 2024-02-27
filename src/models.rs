use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Name(String);

impl Name {
    pub fn new(name: &str) -> Result<Self, String> {
        let size = name.chars().count();
        if size < 1 || size > 10 {
            return Err("名前は10文字以内です".to_string());
        }

        if name.chars().any(|c| !c.is_ascii_alphabetic()) {
            return Err("名前が使用できる文字種はA-Z, a-zです".to_string());
        }

        Ok(Name(name.to_string()))
    }
}

impl FromStr for Name {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Name::new(s)
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
