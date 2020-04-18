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

    #[test]
    fn date_from_str_s() {
        let mut s = "12-04-2000:ghg:".to_string();
        assert_eq!(TagType::date(12,04,2000), TagType::date_from_str(&mut s).unwrap());
        assert_eq!("ghg:".to_string(), s);
    }

    #[test]
    fn date_from_str_f(){
        let mut s = "f0-04-2000:ghg:".to_string();
        assert_eq!(None, TagType::date_from_str(&mut s));
        assert_eq!("f0-04-2000:ghg:".to_string(), s);
    }

    #[test]
    fn number_from_str_s(){
        let mut s = "-2000:ghg:".to_string();
        assert_eq!(TagType::number(-2000), TagType::number_from_str(&mut s).unwrap());
        assert_eq!("ghg:".to_string(), s);
    }

    #[test]
    fn number_from_str_s2(){
        let mut s = "2000:ghg:".to_string();
        assert_eq!(TagType::number(2000), TagType::number_from_str(&mut s).unwrap());
        assert_eq!("ghg:".to_string(), s);
    }

    #[test]
    fn number_from_str_f(){
        let mut s = "t2000:ghg:".to_string();
        assert_eq!(None, TagType::number_from_str(&mut s));
        assert_eq!("t2000:ghg:".to_string(), s);
    }

    #[test]
    fn tags_from_str_s(){
        let mut s = "2000:22-04-2020:ghg:".to_string();
        let tag = Tag::new().append(TagType::str("ghg".to_string())).append(TagType::date(22,04,2020)).append(TagType::number(2000));
        assert_eq!(tag, Tag::from_str(&mut s).unwrap());
        assert_eq!(s, "".to_string());
    }

    #[test]
    fn tags_from_str_s2(){
        let mut s = "2000:22-04-2020:ghg".to_string();
        let tag = Tag::new().append(TagType::str("ghg".to_string())).append(TagType::date(22,04,2020)).append(TagType::number(2000));
        assert_eq!(tag, Tag::from_str(&mut s).unwrap());
        assert_eq!(s, "".to_string());
    }

    #[test]
    fn tags_from_str_s3(){
        let mut s = "2000:22-04-2020:ghg:".to_string();
        let tag = Tag::new().append(TagType::str("ghg".to_string())).append(TagType::date(22,04,2020)).append(TagType::number(2000));
        assert_eq!(tag, Tag::from_str(&mut s).unwrap());
        assert_eq!(s, "".to_string());
    }
}
