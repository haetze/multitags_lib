use std::collections::BTreeSet;
use super::tags::Tag;
use super::query::Query;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct TaggedFile {
    path : String,
    tags : BTreeSet<Tag>,
}


impl TaggedFile {
    pub fn new(p : String, tags : BTreeSet<Tag>) -> Self {
        TaggedFile {
            path : p,
            tags : tags,
        }
    }

    pub fn match_query(&self, query : &Query) -> bool {
        for t in self.tags.iter() {
            if t.match_query(query) {
                return true;
            }
        }
        return false;
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn remove_all_matching(&mut self, query : &Query) {
        let mut s = BTreeSet::new();
        for t in self.tags.iter() {
            if t.match_query(query) {
                s.insert(t.clone());
            }
        }
        for t in s {
            self.tags.remove(&t);
        }
    }

    pub fn add_tag(&mut self, t : Tag) {
        self.tags.insert(t);
    }
    
}


