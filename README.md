# letterboxd-scrap-rust
Rust CLI tool for scraping and extracting movie information from Letterboxd pages.

This is a simple Rust CLI tool that scrapes and extracts movie information from Letterboxd pages. The application is built using the [reqwest](https://docs.rs/reqwest/0.11.4/reqwest/) and [scraper](https://docs.rs/scraper/0.12.0/scraper/) crates.

This isn't a full-fledged application, but rather a simple project to get familiar with Rust and web scraping. 

As of now, the application can extract the following information from a Letterboxd page:
- release year
- director
- rating
- synopsis
- genres

## Usage

To use the application, clone the repository, then you can run the following commands :

```bash
    $cargo run release-year "movie name"
```

```bash
    $cargo run director "movie name"
```

```bash
    $cargo run rating "movie name"
```

```bash
    $cargo run synopsis "movie name"
```

```bash
    $cargo run genres "movie name"
```

```bash
    $cargo run all "movie name"
```
The all command will extract all the information mentioned above.

## Example

```bash
    $cargo run all "the godfather"
```
Result:
```Infos about THE GODFATHER :
Rating: 4.55
Director: Francis Ford Coppola
Release Year: 1972
Synopsis: Spanning the years 1945 to 1955, a chronicle of the fictional Italian-American Corleone crime family. When organized crime family patriarch, Vito Corleone barely survives an attempt on his life, his youngest son, Michael steps in to take care of the would-be killers, launching a campaign of bloody revenge.
Genres: Crime, Drama, Crime, drugs and gangsters, Gritty crime and ruthless gangsters, Violent crime and drugs, Violent action, guns, and crime, Brutal, violent prison drama, Gripping, intense violent crime
```


This a work in progress, and I plan to add more features in the future. 
