mod cli;
mod film_error;
mod film_metadata;
mod film_scraper;

use clap::Parser;
use cli::{Args, Commands};
use film_metadata::FilmMetaData;
use film_scraper::extract_film_meta_datas;

fn display_film_metadata(film_name: &str, metadata: &FilmMetaData) {
    println!("Infos about {} :", film_name.to_uppercase());
    println!("Rating: {:.2}", metadata.rating);
    println!("Director: {}", metadata.director);
    println!("Release Year: {}", metadata.release_year);
    println!("Synopsis: {}", metadata.synopsis);
    if let Some((_, initial_elements)) = metadata.genres.split_last() {
        println!("Genres: {}", initial_elements.join(", "));
    } else {
        println!("Genres: None");
    }
}

fn display_help() {
    println!("Invalid command");
    println!("Usage: letterboxd-cli [command] [film_name]");
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Rate { film_name }) => match extract_film_meta_datas(film_name) {
            Ok(metadata) => {
                println!("Rating: {:.2}", metadata.rating);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        },
        Some(Commands::Director { film_name }) => match extract_film_meta_datas(film_name) {
            Ok(metadata) => {
                println!("Rating: {}", metadata.director);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        },
        Some(Commands::ReleaseYear { film_name }) => match extract_film_meta_datas(film_name) {
            Ok(metadata) => {
                println!("Rating: {}", metadata.release_year);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        },
        Some(Commands::Synopsis { film_name }) => match extract_film_meta_datas(film_name) {
            Ok(metadata) => {
                println!("Rating: {}", metadata.synopsis);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        },
        Some(Commands::Genres { film_name }) => match extract_film_meta_datas(film_name) {
            Ok(metadata) => {
                println!("Genres: {}", metadata.genres.join(", "));
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        },
        Some(Commands::All { film_name }) => match extract_film_meta_datas(film_name) {
            Ok(metadata) => {
                display_film_metadata(&film_name, &metadata);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        },
        None => {
            display_help();
        }
    };
}