use serde::{Serialize, Deserialize};
use super::query::Query;

#[derive(Serialize, Deserialize,Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TagType {
    Date(u8,u8,u32),
    Number(i32),
    Str(String),
}

impl TagType {
    pub fn date(day : u8, month : u8, year : u32) -> Self {
        TagType::Date(day, month, year)
    }

    pub fn number(n : i32) -> Self {
        TagType::Number(n)
    }

    pub fn str(s : String) -> Self {
        TagType::Str(s)
    }
}

#[derive(Serialize, Deserialize,Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tag {
    Nil,
    Cons(TagType, Box<Tag>),
}

impl Tag {
    pub fn new() -> Self {
        Tag::Nil
    }
    pub fn append(self, t : TagType) -> Self {
        Tag::Cons(t, Box::new(self))
    }

    pub fn match_query(&self, query : &Query) -> bool {
        use Query::*;
        match query {
            Or(p, q) => self.match_query(&p) || self.match_query(&q),
            And(p, q) => self.match_query(&p) && self.match_query(&q),
            Eq(t) => t == self,
            Contains(t) => self.contains(t),
            BeginsWith(t) => self.begins_with(t),
            EndsIn(t) => self.ends_in(t),
            _ => false,
        }
    }

    pub fn contains(&self, t : &Tag) -> bool {
        use Tag::*;
        match (self, t) {
            (_, Nil) => true,
            (Cons(a, b), Cons(c, d)) => (a == c && b.begins_with(d)) || b.contains(t),
            _ => false,
        }
    }
    pub fn begins_with(&self, t : &Tag) -> bool {
        use Tag::*;
        match (self, t) {
            (_, Nil) => true,
            (Cons(a, b), Cons(c, d)) => a == c && b.begins_with(d),
            _ => false,
        }
    }
    pub fn ends_in(&self, t : &Tag) -> bool {
        use Tag::*;
        match (self, t) {
            (_, Nil) => true,
            (Cons(a, b), Cons(c, d)) => (a == c && b == d) || b.ends_in(d),
            _ => false,
        }
    }    
}
        



