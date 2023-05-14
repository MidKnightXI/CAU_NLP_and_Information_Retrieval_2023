use serde::Deserialize;
use std::error::Error;
use std::fs;
use reqwest::Client;
use std::env;

#[derive(Debug, Deserialize)]
struct SearchResult {
    results: Vec<SearchResultItem>,
}

#[derive(Debug, Deserialize)]
struct SearchResultItem {
    id: i32,
}

#[derive(Debug, Deserialize)]
struct Filmography {
    cast: Vec<Film>,
}

#[derive(Debug, Deserialize)]
struct Film {
    #[allow(dead_code)]
    title: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let api_key = env::var("API_KEY").expect(
        "API_KEY environment variable not set");

    const URL_BASE: &str = "https://api.themoviedb.org/3";

    let actors_json = fs::read_to_string("actors.json")?;
    let actor_names: Vec<String> = serde_json::from_str(&actors_json)?;

    let mut film_counts: Vec<usize> = vec![0; actor_names.len()];

    for (i, actor_name) in actor_names.iter().enumerate() {
        let actor_url = format!(
            "{}/search/person?api_key={}&query={}",
            URL_BASE,
            api_key,
            actor_name,
        );

        let response = client.get(&actor_url).send().await?.text().await?;
        let search_result: SearchResult = serde_json::from_str(&response)?;

        if let Some(result) = search_result.results.first() {
            let actor_id = result.id;
            let filmography_url = format!(
                "{}/person/{}/movie_credits?api_key={}",
                URL_BASE,
                actor_id,
                api_key
            );

            let filmography_response = client.get(&filmography_url).send().await?.text().await?;
            let filmography: Filmography = serde_json::from_str(&filmography_response)?;

            film_counts[i] = filmography.cast.len();

            println!("Actor: {}, Films: {}", actor_name, film_counts[i]);
        } else {
            println!("Actor not found: {}", actor_name);
        }
    }

    let (max_film_count, actor_with_most_films) = film_counts
        .iter()
        .enumerate()
        .max_by_key(|&(_, count)| count)
        .map(|(i, count)| (count, &actor_names[i]))
        .unwrap();

    println!("Actor with the most films: {}, Films: {}", actor_with_most_films, max_film_count);

    Ok(())
}
