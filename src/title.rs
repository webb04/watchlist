extern crate rustc_serialize;
use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson};

pub struct Title {
    pub name: String,
    pub year: i32,
}

#[derive(RustcDecodable)]
pub struct FetchedTitle {
    pub name: String,
    pub imdb_rating: String,
    pub poster: String
}

impl ToJson for FetchedTitle {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("name".to_string(), self.name.to_json());
        d.insert("imdb_rating".to_string(), self.imdb_rating.to_json());
        d.insert("poster".to_string(), self.poster.to_json());
        Json::Object(d)
    }
}