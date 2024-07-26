use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Rate { film_name: String },
    Director { film_name: String },
    ReleaseYear { film_name: String },
    Synopsis { film_name: String },
    Genres { film_name: String },
    All { film_name: String },
}
