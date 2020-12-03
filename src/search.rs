use std::env;
use crate::title::Title;

pub fn search(title: Title) -> omdb::Movie {
    let api_key = env!("OMDB_API_KEY");
    let show = omdb::title(title.name)
        .apikey(api_key)
        .year(title.year)
        .get()
        .unwrap();

    return show;
}
