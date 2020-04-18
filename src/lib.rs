mod tags;
mod file;
mod query;
mod database;

#[cfg(test)]
mod tests {
    use super::tags::*;

    #[test]
    fn tag_cons() {
        let a = TagType::Date(1,1,2020);
        let b = TagType::Number(1000);
        let c = TagType::Str("vacation".to_string());
        let d = Tag::Nil;
        let e = Tag::Cons(a, Box::new(d));
        let f = Tag::Cons(b, Box::new(e));
        let _g = Tag::Cons(c, Box::new(f));
    }

    #[test]
    fn serialize() {
        let a = TagType::Date(1,1,2020);
        let b = TagType::Number(1000);
        let c = TagType::Str("vacation".to_string());
        let d = Tag::Nil;
        let e = Tag::Cons(a, Box::new(d));
        let f = Tag::Cons(b, Box::new(e));
        let g = Tag::Cons(c, Box::new(f));

        let s = serde_json::to_string(&g).unwrap();
        println!("serialized = {}", s);
    }
    #[test]
    fn deserialize() {
        let a = TagType::Date(1,1,2020);
        let b = TagType::Number(1000);
        let c = TagType::Str("vacation".to_string());
        let d = Tag::Nil;
        let e = Tag::Cons(a, Box::new(d));
        let f = Tag::Cons(b, Box::new(e));
        let g = Tag::Cons(c, Box::new(f));

        let s = serde_json::to_string(&g).unwrap();
        let deserialized: Tag = serde_json::from_str(&s).unwrap();

        println!("deserialized = {:?}", deserialized);

        assert_eq!(deserialized, g);
    }
}
