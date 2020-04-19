use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

use super::tags::Tag;
use super::query::Query;
use super::file::TaggedFile;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct DB {
    file : String,
    db : HashSet<TaggedFile>,
}


impl DB {
    pub fn init(file : String) -> Self {
        DB {
            file : file,
            db : HashSet::new(),
        }
    }

    pub fn read_db(file : String) -> std::io::Result<Self> {
        let mut file = File::open(file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let s = serde_json::from_str(&contents)?;
        Ok(s)
    }

    pub fn dump_db(&self) -> std::io::Result<()> {
        let mut f = File::create(self.file.clone())?;
        let s = serde_json::to_string(self)?;
        f.write_all(s.as_bytes())?;
        Ok(())
    }

    pub fn match_query(&self, query : &Query) -> HashSet<String> {
        let mut set = HashSet::new();
        for tf in self.db.iter() {
            if tf.match_query(query) {
                set.insert(tf.get_path());
            }
        }
        return set;
    }

    pub fn remove_matching(&mut self, query: &Query) {
        let mut s : HashSet<TaggedFile>= HashSet::new();
        
        for tf in self.db.iter() {
            if tf.match_query(query) {
                s.insert(tf.clone());
            }
        }
        
        for tf in s.iter() {
            self.db.remove(tf);
        }
    }

    pub fn remove_matching_tags(&mut self, query: &Query) {
        let s = self.db.clone();
        for ts in s {
            let mut t = self.db.take(&ts).unwrap();
            t.remove_all_matching(query);
            self.db.insert(t);
        }
    }

    pub fn remove_matching_tags_for_file(&mut self, file : &String, query : &Query) {
     let s = self.db.clone();
        for ts in s {
            let mut t = self.db.take(&ts).unwrap();
            if &t.get_path() == file {
                t.remove_all_matching(query);
            }
            self.db.insert(t);
        }
    }

    pub fn add_tag_matching(&mut self, query : &Query, tag : &Tag) {
        let mut s : HashSet<TaggedFile>= HashSet::new();
        
        for tf in self.db.iter() {
            if tf.match_query(query) {
                s.insert(tf.clone());
            }
        }
        
        for tf in s.iter() {
            let mut t = self.db.take(tf).unwrap();
            t.add_tag(tag.clone());
            self.db.insert(t);
        }
    }

    pub fn add_tag_to_file(&mut self, file : String, tag : &Tag) {
        let s = self.db.clone();
        for tf in s {
            let mut t = self.db.take(&tf).unwrap();
            if t.get_path() == file {
                t.add_tag(tag.clone());
            }
            self.db.insert(t);
        }
    }
        

}



