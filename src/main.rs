use sqlx::{migrate::MigrateDatabase,Sqlite, SqlitePool};
use sqlx::sqlite::SqliteQueryResult;
 const DB_URL:&str = "sqlite://sqlite.db";


// async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
//     let pool = SqlitePool::connect(&db_url).await?;
//     let qry =
//         "
//     ";
//
//
//     let result = sqlx::query(&qry)
//         .execute(&pool).await;
//     println!("----------");
//     pool.close().await;
//     return result
// }


#[tokio::main]
async fn main() {
    //let db_url = String::from("sqlite://sqlite.db");
    // db migration
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

    // if !Sqlite::database_exists(&DB_URL).await.unwrap_or(false) {
    //     Sqlite::create_database(&DB_URL).await.unwrap();
    //     match  create_schema(&DB_URL).await  {
    //         Ok(_) => println!("Schema created"),
    //         Err(e) => println!("Error creating schema: {:?}", e)
    //     }
    // }




    let instance = SqlitePool::connect(&DB_URL).await.unwrap();
    let qry = "INSERT INTO settings (description) VALUES($1)";
    let result = sqlx::query(&qry)
        .bind("testing")
        .execute(&instance).await;

    instance.close().await;
    println!("{:?}", result);
}