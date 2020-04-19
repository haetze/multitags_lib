use std::collections::BTreeSet;
use super::tags::Tag;
use super::query::Query;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
/// TaggedFile holds the path to a file and all tags that were added to this file.
pub struct TaggedFile {
    path : String,
    tags : BTreeSet<Tag>,
}


impl TaggedFile {
    /// Creates a new TaggedFile from a String and a BTreeSet of initial Tags. 
    pub fn new(p : String, tags : BTreeSet<Tag>) -> Self {
        TaggedFile {
            path : p,
            tags : tags,
        }
    }

    /// Tries to match a query with a tag in `self`.
    pub fn match_query(&self, query : &Query) -> bool {
        for t in self.tags.iter() {
            if t.match_query(query) {
                return true;
            }
        }
        return false;
    }

    /// Returns a clone of the `path`-Field.
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    /// Removes all tags that match `query`.
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

    /// Adds `t` to the `tags`.
    pub fn add_tag(&mut self, t : Tag) {
        self.tags.insert(t);
    }
    
}


