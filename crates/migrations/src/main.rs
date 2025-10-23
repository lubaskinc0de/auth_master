use postgres::{Client, NoTls};
use shared::config;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

fn main() {
    let config = config::from_env();
    let mut conn = Client::connect(&config.db.url, NoTls).unwrap();
    embedded::migrations::runner().run(&mut conn).unwrap();
    println!("Migrations completed");
}
