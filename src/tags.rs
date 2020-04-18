use serde::{Serialize, Deserialize};
use super::query::Query;

#[derive(Serialize, Deserialize,Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TagType {
    Date(u32,u32,u32),
    Number(i32),
    Str(String),
}

impl TagType {
    pub fn date(day : u32, month : u32, year : u32) -> Self {
        TagType::Date(day, month, year)
    }

    pub fn number(n : i32) -> Self {
        TagType::Number(n)
    }

    pub fn str(s : String) -> Self {
        TagType::Str(s)
    }

    pub fn date_from_str(s : &mut String) -> Option<Self> {
        match s.clone().chars().collect::<Vec<char>>()[..] {
            [d1,d2,'-',m1,m2,'-',y1,y2,y3,y4,':',..] => {
                let d1 = d1.to_digit(10)?;
                let d2 = d2.to_digit(10)?;
                let m1 = m1.to_digit(10)?;
                let m2 = m2.to_digit(10)?;
                let y1 = y1.to_digit(10)?;
                let y2 = y2.to_digit(10)?;
                let y3 = y3.to_digit(10)?;
                let y4 = y4.to_digit(10)?;
                let mut s2 = s.split_off(11);
                std::mem::swap(s, &mut s2);
                Some(TagType::date(d1*10+d2, m1*10+m2,y1*1000+y2*100+y3*10+y4))
            },
            _ => None,
        }
    }

    pub fn number_from_str(s : &mut String) -> Option<Self> {
        match s.find(':') {
            None => None,
            Some(n) => {
                let mut s1 = s.clone();
                let mut s2 = s1.split_off(n);
                std::mem::swap(&mut s1, &mut s2);
                let x : i32 = s2.parse().ok()?;
                s1.remove(0);
                std::mem::swap(s, &mut s1);
                Some(TagType::number(x))
            }
        }
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

    pub fn from_str(s : &mut String) -> Option<Self> {
        let mut tags : Vec<TagType> = Vec::new();
        loop {
            if let Some(t) = TagType::date_from_str(s) {
                tags.push(t);
            } else if let Some(t) = TagType::number_from_str(s) {
                tags.push(t);
            } else {
                match s.find(':') {
                    None => {
                        tags.push(TagType::Str(s.clone()));
                        s.clear();
                        break;
                    },
                    Some(n) => {
                        let mut s1 = s.split_off(n);
                        tags.push(TagType::Str(s.clone()));
                        s1.remove(0);
                        std::mem::swap(s, &mut s1);
                    },
                }
            }

            if s.len() == 0 {
                break;
            }
        }

        let mut t = Tag::new();
        while let Some(tagt) = tags.pop() {
            t = t.append(tagt);
        }

        return Some(t);
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
        



