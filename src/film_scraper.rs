use crate::film_error::FilmError;
use crate::film_metadata::FilmMetaData;
use scraper::{Html, Selector};

fn get_html_content(url: &str) -> Result<String, FilmError> {
    let response = reqwest::blocking::get(url)?;
    Ok(response.text()?)
}

fn build_film_url(film_name: &str) -> String {
    let film_name = film_name.to_lowercase().replace(" ", "-");
    return format!("https://letterboxd.com/film/{}", film_name);
}

fn fetch_and_parse_html(film_name: &str) -> Result<Html, FilmError> {
    let film_url = build_film_url(film_name);
    let html_content = get_html_content(&film_url)?;
    let document = Html::parse_document(&html_content);
    Ok(document)
}

fn get_film_rating(document: &Html, selector: &Selector) -> Result<f32, FilmError> {
    let element = document
        .select(selector)
        .next()
        .ok_or(FilmError::SelectorError)?;

    let content = element
        .value()
        .attr("content")
        .ok_or(FilmError::AttributeError)?;

    let rating: f32 = content
        .split_whitespace()
        .next()
        .ok_or(FilmError::ParseError)?
        .parse()
        .map_err(|_| FilmError::ParseError)?;

    Ok(rating)
}

fn get_film_single_info(document: &Html, selector: &Selector) -> Result<String, FilmError> {
    let info = document
        .select(selector)
        .next()
        .ok_or(FilmError::SelectorError)?
        .text()
        .collect::<String>();
    Ok(info)
}

fn get_film_multiples_infos(
    document: &Html,
    selector: &Selector,
) -> Result<Vec<String>, FilmError> {
    let infos = document.select(selector);
    let mut infos_vec = Vec::new();

    for info in infos {
        infos_vec.push(info.text().collect::<String>());
    }

    Ok(infos_vec)
}

fn extract_release_year(document: &Html) -> Result<String, FilmError> {
    let selector = Selector::parse("div.releaseyear a").unwrap();
    get_film_single_info(document, &selector)
}

fn extract_director(document: &Html) -> Result<String, FilmError> {
    let selector = Selector::parse("span.directorlist a span.prettify").unwrap();
    get_film_single_info(document, &selector)
}

fn extract_synopsis(document: &Html) -> Result<String, FilmError> {
    let selector = Selector::parse("div.truncate p").unwrap();
    get_film_single_info(document, &selector)
}
fn extract_rating(document: &Html) -> Result<f32, FilmError> {
    let selector = Selector::parse(r#"meta[name="twitter:data2"]"#).unwrap();
    get_film_rating(document, &selector)
}

fn extract_genres(document: &Html) -> Result<Vec<String>, FilmError> {
    let selector = Selector::parse("div.text-sluglist.capitalize a.text-slug").unwrap();
    get_film_multiples_infos(document, &selector)
}

pub fn extract_film_meta_datas(film_name: &str) -> Result<FilmMetaData, FilmError> {
    let document = fetch_and_parse_html(film_name)?;

    let release_year = extract_release_year(&document)?;
    let director = extract_director(&document)?;
    let synopsis = extract_synopsis(&document)?;
    let rating = extract_rating(&document)?;
    let genres = extract_genres(&document)?;

    Ok(FilmMetaData {
        rating,
        director,
        release_year,
        synopsis,
        genres,
    })
}
