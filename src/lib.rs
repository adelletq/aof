use chrono::{DateTime,Utc};
use serde::{Serialize,Deserialize};

#[derive(Clone,Debug,Eq,PartialEq,Serialize,Deserialize)]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[derive(Clone,Debug,Eq,PartialEq,Serialize,Deserialize)]
pub enum ElementKind {
    H2,
    H3,
    H4,
    P
}

#[derive(Clone,Debug,Eq,PartialEq,Serialize,Deserialize)]
pub struct Link {
    pub content: String,
    pub destination: String,
}

#[derive(Clone,Debug,Eq,PartialEq,Serialize,Deserialize)]
pub enum Element {
    Simple { kind: ElementKind, content: String,},
    Image { src: String, link: Link,},
}

#[derive(Clone,Debug,Eq,PartialEq,Serialize,Deserialize)]
pub struct BlogEntry {
    pub id: i32,
    pub title: String,
    pub image_url: String,
    pub author: Author,
    pub created_on: DateTime<Utc>,
    pub description: String,
//     pub contents: Vec<Element>,
}
