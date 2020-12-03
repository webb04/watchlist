mod database;
mod search;
mod title;
use gotham::helpers::http::response::create_response;
use gotham::hyper::header::CONTENT_TYPE;
use gotham::hyper::StatusCode;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;
use title::Title;
use std::fs;
use gotham::handler::HandlerResult;
use crate::title::FetchedTitle;
extern crate rustc_serialize;
use rustc_serialize::json::{self, Json, ToJson};

extern crate gotham;

fn generate_view(title: FetchedTitle) -> String {
    let movie_template = fs::read_to_string("views/movie.html")
            .expect("Something went wrong reading the file")
            .replace("{{ NAME }}", &title.name)
            .replace("{{ RATING }}", &title.imdb_rating)
            .replace("{{ IMG }}", &title.poster);
    return movie_template.to_string();
}

pub async fn handler(state: State) -> HandlerResult {
    let titles = database::fetch().await.unwrap();
    let string = titles
        .into_iter()
        .map(|title| generate_view(title))
        .collect::<Vec<_>>()
        .join("\n");

    let mime = mime::TEXT_HTML;
    let contents = fs::read_to_string("views/index.html")
        .expect("Something went wrong reading the file")
        .replace("{{ MOVIES }}", &string);

    let mut res = create_response(&state, StatusCode::OK, mime, contents);

    {
        let headers = res.headers_mut();
        headers.insert(CONTENT_TYPE, "text/html".parse().unwrap());
    };

    Ok((state, res))
}

pub async fn movies_handler(state: State) -> HandlerResult {
    let titles = database::fetch().await.unwrap();
    let encoded = json::encode(&titles.to_json()).unwrap();
    let mime = mime::APPLICATION_JSON;
    let mut res = create_response(&state, StatusCode::OK, mime, encoded);

    {
        let headers = res.headers_mut();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    };

    Ok((state, res))
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to_async(handler);
        route.get("/movies").to_async(movies_handler);
    })
}

fn main() {
    setup();
    let addr = "127.0.0.1:1616";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

fn setup() {
    let _result = database::setup();

    let titles: Vec<Title> = vec![
        Title {
            name: "Tenet".to_string(),
            year: 2020,
        },
        Title {
            name: "Onward".to_string(),
            year: 2020,
        },
        Title {
            name: "Oldboy".to_string(),
            year: 2003,
        },
        Title {
            name: "Mulan".to_string(),
            year: 2020,
        },
    ];

    let movies: Vec<omdb::Movie> = titles
        .into_iter()
        .map(|title| search::search(title))
        .collect();

    for movie in movies {
        let res: Result<(), postgres::Error> = database::insert(movie);
        match res {
            Ok(_) => println!("Inserted into database"),
            Err(e) => println!("Error inserting into database: {}", e),
        }
    }
}
