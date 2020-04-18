use super::tags::Tag;

pub enum Query {
    Or(Box<Query>, Box<Query>),
    And(Box<Query>, Box<Query>),
    Eq(Tag),
    Contains(Tag),
    EndsIn(Tag),
    BeginsWith(Tag),
}


impl Query {
    pub fn eq_c(t : Tag) -> Self {
        Query::Eq(t)
    }
    pub fn contains_c(t : Tag) -> Self {
        Query::Contains(t)
    }
    pub fn ends_in_c(t : Tag) -> Self {
        Query::EndsIn(t)
    }
    pub fn begins_with_c(t : Tag) -> Self {
        Query::BeginsWith(t)
    }
    pub fn or_c(q : Query, p: Query) -> Self {
        Query::Or(Box::new(q), Box::new(p))
    }
    pub fn and_c(q : Query, p: Query) -> Self {
        Query::And(Box::new(q), Box::new(p))
    }
}
