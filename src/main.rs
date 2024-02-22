use sqlx::{migrate::MigrateDatabase,Sqlite, SqlitePool};
use sqlx::sqlite::SqliteQueryResult;

// const DB_URL:&str = "sqlite://sqlite.db";


async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(&db_url).await?;
    let qry =
        "PRAGMA foreign_keys = ON ;
    CREATE TABLE IF NOT EXISTS settings
        (
            settings_id             INTEGER PRIMARY KEY NOT NULL,
            description             TEXT                NOT NULL,
            created_on              DATETIME DEFAULT (datetime('now','localtime')),
            updated_on              DATETIME DEFAULT (datetime('now','localtime')),
            done                    BOOLEAN             NOT NULL DEFAULT 0
        );
    CREATE TABLE IF NOT EXISTS project
        (
            project_id                   INTEGER PRIMARY KEY AUTOINCREMENT,
            product_name                 TEXT ,
            created_on                   DATETIME DEFAULT (datetime('now','localtime')),
            updated_on                   DATETIME DEFAULT (datetime('now','localtime')),
            img_directory                TEXT NOT NULL,
            out_directory                TEXT NOT NULL,
            status                       TEXT NOT NULL,
            settings_id                  INTEGER  NOT NULL DEFAULT 1,
            FOREIGN KEY (settings_id)    REFERENCES settings (settings_id) ON UPDATE SET NULL ON DELETE SET NULL
        );";


    let result = sqlx::query(&qry)
        .execute(&pool).await;
    println!("----------");
    pool.close().await;
    return result
}


#[tokio::main]
async fn main() {
    let db_url = String::from("sqlite://sqlite.db");
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap();
        match  create_schema(&db_url).await  {
            Ok(_) => println!("Schema created"),
            Err(e) => println!("Error creating schema: {:?}", e)
        }
    }
    let instance = SqlitePool::connect(&db_url).await.unwrap();
    let qry = "INSERT INTO settings (description) VALUES($1)";
    let result = sqlx::query(&qry)
        .bind("testing")
        .execute(&instance).await;

    instance.close().await;
    println!("{:?}", result);
}