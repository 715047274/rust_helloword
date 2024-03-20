use sqlx::{migrate::MigrateDatabase,Sqlite, SqlitePool};
use sqlx::sqlite::SqliteQueryResult;
 const DB_URL:&str = "sqlite://sqlite.db";


#[tokio::main]
async fn main() {

    if !Sqlite::database_exists(&DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", &DB_URL);
        match Sqlite::create_database(&DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let db = SqlitePool::connect(&DB_URL).await.unwrap();

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");

    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db)
        .await;

    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }

    println!("migration: {:?}", migration_results);
    let instance = SqlitePool::connect(&DB_URL).await.unwrap();
    let qry = "INSERT INTO settings (description) VALUES($1)";
    let result = sqlx::query(&qry)
        .bind("testing")
        .execute(&instance).await;

    instance.close().await;
    println!("{:?}", result);
}