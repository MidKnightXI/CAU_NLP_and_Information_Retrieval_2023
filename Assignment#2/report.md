# Natural Language and Processing Assignment #2

The goal of this assignment is to retrieve the most popular actor of a country based on the number of films he made or the popularity of him.

As I'm french I decided to try to get the most famous french actor. But I decided that my code should also be compatible for other nationalities so that the code could be reusable for maybe later usages.

Also for the technology I'll use, I haven't choosen the easiest possible as overall my code would be pretty easy. So instead of JavaScript or Python, I decided to use Rust, a language that is aiming for memory-safety and performances. One of the primary difficulties of Rust is its steep learning curve. Rust introduces a unique set of concepts and features, such as ownership, borrowing, and lifetimes, which can be quite different from traditional programming languages. This learning curve requires developers to invest time and effort in understanding and mastering these concepts before they can effectively utilize Rust's full potential.

To get the informations about the actors I decided to not use IMDb but [TMDB](https://www.themoviedb.org/) because IMDb requires an Amazon account to access its API and its documentation is overall pretty confusing as they are using GraphQL instead of a regular Rest API. TMDB is pretty well documented and easy to access for developpers as it requires you to just create a new account and fill-up some informations about the usage you will have with their API.

For the requests to the TMDB API I used the crates (this is the name of the libraries in Rust) [tokio](https://crates.io/crates/tokio) to run some operations asynchronously and [reqwest](https://crates.io/crates/reqwest) for the HTTP requests to the API.

# The code

The code first imports the necessary dependencies and defines several data structures using the serde library for deserialization. These structures represent the search results, individual search items, filmographies, and films.

The main function sets up a client for making HTTP requests and retrieves the API key from the environment variables. It also defines the base URL for the TMDb API.

The code reads the contents of a file named "[actors.json](#actors)" which is expected to contain a JSON array of actor names. It deserializes this JSON array into a vector of strings.

Next, the code initializes a vector called film_counts with a length equal to the number of actor names, setting all counts to 0.

The code then iterates over each actor name, constructing a URL to search for the actor using the TMDb API. It makes an HTTP GET request to the API and retrieves the response. The response is deserialized into a SearchResult struct. If a search result is found, it extracts the actor's ID and constructs another URL to retrieve the filmography of that actor. Another HTTP request is made, and the response is deserialized into a Filmography struct. The number of films in the filmography is stored in the corresponding index of the film_counts vector.

For each actor, the code prints their name and the number of films they have appeared in.

After processing all actors, the code finds the maximum film count and the corresponding actor name using the max_by_key function. It prints the actor with the most films and the number of films they have appeared in.

Finally, the code returns Ok(()) to indicate successful execution or an error if any occurred during the process.

```rust
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
```

<a id="actors"></a>
Overall, this JSON represents a collection of talented individuals who have made significant contributions to French cinema. Their performances and artistic achievements have solidified their places as some of the most respected and admired figures in the industry.

```json
[
    "Jean-Louis Trintignant",
    "Marion Cotillard",
    "Omar Sy",
    "Léa Seydoux",
    "Vincent Cassel",
    "Louis Garrel",
    "Isabelle Huppert",
    "Gaspard Ulliel",
    "Audrey Tautou",
    "Mélanie Laurent",
    "Romain Duris",
    "Juliette Binoche",
    "Guillaume Canet",
    "Charlotte Gainsbourg",
    "Olivier Martinez",
    "Eva Green",
    "Mathieu Amalric",
    "Emmanuelle Béart",
    "François Cluzet",
    "Catherine Deneuve",
    "Romain Duris",
    "Eva Green",
    "Louis-Julien Petit",
    "Vincent Lindon",
    "Kad Merad",
    "Jean Reno",
    "Michel Blanc",
    "Vincent Perez",
    "Olivier Gourmet",
    "Pierre Niney",
    "Leïla Bekhti",
    "Reda Kateb",
    "Sara Forestier",
    "Denis Ménochet",
    "Ludivine Sagnier",
    "Gérard Depardieu",
    "Daniel Auteuil",
    "Jean-Pierre Bacri",
    "Emmanuelle Devos",
    "Christophe Lambert",
    "Mélanie Thierry",
    "Niels Arestrup",
    "Jacques Audiard",
    "André Dussollier",
    "Sandrine Kiberlain",
    "José Garcia",
    "Raphaël Personnaz",
    "Jean-Paul Belmondo",
    "Emmanuelle Seigner",
    "Vincent Macaigne",
    "Adèle Exarchopoulos",
    "Vincent Perez",
    "Marina Foïs",
    "Gilles Lellouche",
    "Olivier Rabourdin",
    "Cécile de France",
    "Karin Viard",
    "Roschdy Zem"
]
```

# Results

Here is the output of the code when running:

```
Actor: Jean-Louis Trintignant, Films: 154
Actor: Marion Cotillard, Films: 94
Actor: Omar Sy, Films: 75
Actor: Léa Seydoux, Films: 48
Actor: Vincent Cassel, Films: 99
Actor: Louis Garrel, Films: 57
Actor: Isabelle Huppert, Films: 172
Actor: Gaspard Ulliel, Films: 47
Actor: Audrey Tautou, Films: 41
Actor: Mélanie Laurent, Films: 52
Actor: Romain Duris, Films: 58
Actor: Juliette Binoche, Films: 108
Actor: Guillaume Canet, Films: 73
Actor: Charlotte Gainsbourg, Films: 81
Actor: Olivier Martinez, Films: 23
Actor: Eva Green, Films: 32
Actor: Mathieu Amalric, Films: 136
Actor: Emmanuelle Béart, Films: 67
Actor: François Cluzet, Films: 89
Actor: Catherine Deneuve, Films: 195
Actor: Romain Duris, Films: 58
Actor: Eva Green, Films: 32
Actor: Louis-Julien Petit, Films: 0
Actor: Vincent Lindon, Films: 80
Actor: Kad Merad, Films: 104
Actor: Jean Reno, Films: 97
Actor: Michel Blanc, Films: 83
Actor: Vincent Perez, Films: 71
Actor: Olivier Gourmet, Films: 115
Actor: Pierre Niney, Films: 35
Actor: Leïla Bekhti, Films: 46
Actor: Reda Kateb, Films: 44
Actor: Sara Forestier, Films: 36
Actor: Denis Ménochet, Films: 44
Actor: Ludivine Sagnier, Films: 52
Actor: Gérard Depardieu, Films: 245
Actor: Daniel Auteuil, Films: 104
Actor: Jean-Pierre Bacri, Films: 51
Actor: Emmanuelle Devos, Films: 83
Actor: Christophe Lambert, Films: 6
Actor: Mélanie Thierry, Films: 41
Actor: Niels Arestrup, Films: 63
Actor: Jacques Audiard, Films: 9
Actor: André Dussollier, Films: 137
Actor: Sandrine Kiberlain, Films: 74
Actor: José Garcia, Films: 65
Actor: Raphaël Personnaz, Films: 44
Actor: Jean-Paul Belmondo, Films: 101
Actor: Emmanuelle Seigner, Films: 43
Actor: Vincent Macaigne, Films: 53
Actor: Adèle Exarchopoulos, Films: 36
Actor: Vincent Perez, Films: 71
Actor: Marina Foïs, Films: 68
Actor: Gilles Lellouche, Films: 66
Actor: Olivier Rabourdin, Films: 65
Actor: Cécile de France, Films: 53
Actor: Karin Viard, Films: 101
Actor: Roschdy Zem, Films: 91
Actor with the most films: Gérard Depardieu, Films: 245
```

And as we can see here is the result, Gérard Depardieu is the french actor that appeared in the most movies.

```
Actor with the most films: Gérard Depardieu, Films: 245
```

# Improvements

To improve this code we could add a kind of heuristic because not every movies are the same, a blockbuster shouldn't value the same as a small film realized by an independant.

Also we could add a way to retrieve actors directly from TMDB instead of using a json even if its still pretty convenient for the user anyway for both methods.

Adding a ranking or a graph to show the overall ranking could be a good way fro the user to have a perception overall of the other actors.