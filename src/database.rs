use postgres::{Client as syncClient, Error as syncError, NoTls as syncNoTls};
use tokio_postgres::{NoTls, Error};
use crate::FetchedTitle;

pub fn setup() -> Result<(), syncError> {
    let url = env!("POSTGRES");
    let mut client = syncClient::connect(url, syncNoTls)?;
    client.batch_execute("DROP TABLE IF EXISTS titles;")?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS titles (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            imdb_rating     VARCHAR NOT NULL,
            poster          VARCHAR NOT NULL
            )
    ",
    )?;
    Ok(())
}

pub fn insert(movie: omdb::Movie) -> Result<(), syncError> {
    let url = env!("POSTGRES");
    let mut client = syncClient::connect(url, syncNoTls)?;

    client.batch_execute(
        &format!(
            "
            INSERT INTO titles (name, imdb_rating, poster) VALUES
                ('{}', '{}', '{}')
        ",
            movie.title, movie.imdb_rating, movie.poster
        )
        .to_string(),
    )?;

    Ok(())
}

pub async fn fetch() -> Result<Vec<FetchedTitle>, Error> {
    let url = env!("POSTGRES");
    let (client, connection) =
        tokio_postgres::connect(url, NoTls).await?;

    let mut vec = Vec::new();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    for row in client.query("SELECT name, imdb_rating, poster FROM titles", &[]).await? {
        let name: &str = row.get(0);
        let rating: &str = row.get(1);
        let poster: &str = row.get(2);
        vec.push(FetchedTitle {
            name: name.to_string(),
            imdb_rating: rating.to_string(),
            poster: poster.to_string()
        });
    }

    Ok(vec)
}