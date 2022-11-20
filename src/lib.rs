use chrono::{DateTime, Utc};

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct BlogEntry {
    pub id: i32,
    pub title: String,
    pub image_url: String,
    pub author: Author,
    pub when: DateTime<Utc>,
    pub description: String,
    pub contents: String,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
