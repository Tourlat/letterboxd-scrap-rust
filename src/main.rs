use std::{fs::File, io::Write};

use scraper::{Html, Selector};
use serde_json::Value;

const URL_WATCHLIST: &str =
    "https://letterboxd.com/film/scouts-honor-the-secret-files-of-the-boy-scouts-of-america/";
const URL: &str = "https://letterboxd.com/terracid/";

// let response2 = reqwest::blocking::get(URL);
// let html_content2 = response2.unwrap().text().unwrap();

// let mut file = File::create("output.html").unwrap();
// file.write_all(html_content2.as_bytes()).unwrap();

fn main() {
    let response = reqwest::blocking::get(URL);
    let html_content = response.unwrap().text().unwrap();

    let response_watchlist = reqwest::blocking::get(URL_WATCHLIST);
    let html_content_watchlist = response_watchlist.unwrap().text().unwrap();

    let mut file = File::create("output.html").unwrap();
    file.write_all(html_content_watchlist.as_bytes()).unwrap();

    let document_film = Html::parse_document(&html_content_watchlist);

    let selector_release_year = Selector::parse("div.releaseyear a").unwrap();
    let selector_director = Selector::parse("span.directorlist a span.prettify").unwrap();
    let selector_synopsis = Selector::parse("div.truncate p").unwrap();
    let selector_rating = Selector::parse(r#"meta[name="twitter:data2"]"#).unwrap();
    let selector_genre = Selector::parse("div.text-sluglist.capitalize a.text-slug").unwrap();
   
    let release_year = document_film
        .select(&selector_release_year)
        .next()
        .unwrap()
        .text()
        .collect::<String>();
    let director_name = document_film
        .select(&selector_director)
        .next()
        .unwrap()
        .text()
        .collect::<String>();
    let synopsis = document_film
        .select(&selector_synopsis)
        .next()
        .unwrap()
        .text()
        .collect::<String>();

    let genres = document_film
        .select(&selector_genre);

    for genre in genres {
        println!("{}", genre.text().collect::<String>());
    }


    

    println!("Release Year: {}", release_year);
    println!("Director: {}", director_name);
    println!("Synopsis: {}", synopsis);


    if let Some(element) = document_film.select(&selector_rating).next() {
        if let Some(content) = element.value().attr("content") {
            println!("Found rating: {}", content);
            // If you need to extract the numerical value only:
            let rating: f32 = content.split_whitespace().next().unwrap().parse().unwrap();
            println!("Numerical rating value: {}", rating);
        }
    }

    let film_selector = Selector::parse("li.favourite-film-poster-container img").unwrap();
    let mut film_names = Vec::new();

    let document = Html::parse_document(&html_content);

    for film in document.select(&film_selector) {
        let film_name = film.value().attr("alt").unwrap();
        film_names.push(film_name);
    }

    for name in film_names {
        println!("{}", name);
    }
}
