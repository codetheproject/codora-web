use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct UserName(String);

#[derive(Debug)]
pub struct Error(String);

type Store = HashMap<String, Vec<Error>>;

// type Error =  Vec<Error> | HashMap<String, Vec<Error>>

#[derive(Debug)]
struct User {
    username: UserName,
}

impl FromStr for UserName {
    type Err = Store;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let username = UserName(s.into());
        Ok(username)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_username() {
        let username = "West".parse::<UserName>();

        println!("{:?}", username);
    }
}
