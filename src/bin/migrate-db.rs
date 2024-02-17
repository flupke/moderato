use moderato::db;

pub fn main() {
    match db::migrate() {
        Ok(_) => println!("Database migrated"),
        Err(error) => println!("Error migrating database: {}", error),
    }
}
