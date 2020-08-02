use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;

use super::tags::Tag;
use super::query::Query;
use super::file::TaggedFile;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
/// Representation of the Tag-Database at runtime.
pub struct DB {
    file : String,
    db : HashMap<String, TaggedFile>,
}


impl DB {
    /// Creates a new database from the location string.
    pub fn init(file : String) -> Self {
        DB {
            file : file,
            db : HashMap::new(),
        }
    }
    /// Reads a database at `file`.
    pub fn read_db(file : String) -> std::io::Result<Self> {
        let mut file = File::open(file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let s = serde_json::from_str(&contents)?;
        Ok(s)
    }

    /// Writes the database to `self.file`.
    pub fn dump_db(&self) -> std::io::Result<()> {
        let mut f = File::create(self.file.clone())?;
        let s = serde_json::to_string(self)?;
        f.write_all(s.as_bytes())?;
        Ok(())
    }

    /// Tries to match `query` with every TaggedFile.
    /// Returns all paths for files that positively match with the query.
    pub fn match_query(&self, query : &Query) -> HashSet<String> {
        self.db
            .iter()
            .filter(|(_, tf)| tf.match_query(query))
            .map(|p| p.0.clone()).collect()
    }

    /// Removes TaggedFiles that match `query`.
    pub fn remove_matching(&mut self, query: &Query) {
        for (p, _) in self.db.clone().iter().filter(|(_, tf)| tf.match_query(query)) {
            self.db.remove(p);
        }
    }

    /// Removes Tags for each TaggedFile.
    /// Removed Tags match `query` positively.
    pub fn remove_matching_tags(&mut self, query: &Query) {
        for (_p,ts) in self.db.iter_mut() {
            ts.remove_all_matching(query);
        }
    }

    /// Removes Tags for `file`.
    /// Removed Tags match `query` positively.
    pub fn remove_matching_tags_for_file(&mut self, file : &String, query : &Query) {
        match self.db.remove(file) {
            None => {
            },
            Some(mut tf) => {
                tf.remove_all_matching(query.clone());
                self.db.insert(file.to_string(), tf);
            },
        }
    }

    /// Adds `tag` to each TaggedFile that matches `query`.
    pub fn add_tag_matching(&mut self, query : &Query, tag : &Tag) {
        for (_p,tf) in self.db.iter_mut() {
            if tf.match_query(query) {
                tf.add_tag(tag.clone());
            }
        }
    }

    /// Adds `tag` to `file.`
    pub fn add_tag_to_file(&mut self, file : String, tag : &Tag) {
        match self.db.remove(&file) {
            None => {
            },
            Some(mut tf) => {
                tf.add_tag(tag.clone());
                self.db.insert(file, tf);
            },
        }
    }

    /// Adds `file` with an empty Tag-Set.
    pub fn add_file(&mut self, file : String) {
        match self.db.get(&file) {
            None => {
                self.db.insert(file.clone(), TaggedFile::new(file, BTreeSet::new()));
            },
            Some(_) => {
            },
        }
        return;

    }   
        

}



