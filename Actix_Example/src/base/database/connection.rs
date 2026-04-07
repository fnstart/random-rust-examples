use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement};

pub struct Db;

impl Db {
    pub async fn connect() -> DatabaseConnection {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let db = Database::connect(&url)
            .await
            .expect("Failed to connect to SQLite");

        db.execute(Statement::from_string(
            DbBackend::Sqlite,
            "CREATE TABLE IF NOT EXISTS users (
                id       INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT    NOT NULL,
                email    TEXT    NOT NULL UNIQUE
            )"
            .to_string(),
        ))
        .await
        .expect("Failed to create users table");

        db
    }
}
