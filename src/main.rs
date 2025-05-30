#[macro_use] // This is a macro that allows us to use the `rocket` crate
extern crate rocket; // This is the main crate for the Rocket web framework
use rocket::serde::{json::Json, Serialize, Deserialize}; // This is a macro that allows us to use the `rocket` crate
use rocket::State;  // This is a macro that allows us to use the `State` trait
use rocket::response::content::RawHtml; // This is a macro that allows us to use the `Html` trait
use sqlx::SqlitePool; // This is a macro that allows us to use the `SqlitePool` trait
use tera::{Tera, Context}; // This is a macro that allows us to use the `Tera` trait

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
struct Greeting{
    id: i32,
    name: String,
    message: String,
}

#[derive(Deserialize)]
struct NewGreeting {
    name: String,
    message: String,
}

#[derive(Serialize)] // This is a macro that allows us to serialize the `Message` struct
struct Message {
    message: String,
}

#[get("/")] // This is a macro that allows us to get the `hello_world` function
fn hello_world() -> Json<Message> {
    Json(Message {
        message: String::from("Hello, world!"),
    })
}

#[get("/hello/<n>")] // This is a macro that allows us to get the `hello_name` function
fn hello_name(n: &str) -> Json<Message> {
    Json(Message {
        message: format!("Hello, {}!", n),
    })
}

// POST 
#[post("/greeting", data = "<greeting>")]
async fn create_greeting(greeting: Json<NewGreeting>, pool: &State<SqlitePool>) -> Json<Message>{
    sqlx::query("INSERT INTO greetings (name, message) VALUES (?, ?)")
        .bind(&greeting.name)
        .bind(&greeting.message)
        .execute(pool.inner())
        .await
        .unwrap();
    Json(Message {
        message: String::from("Greeting Created!"),
    })
}

#[get("/greetings")]
async fn get_greetings(pool: &State<SqlitePool>) -> Json<Vec<Greeting>> {
    let greetings = sqlx::query_as::<_, Greeting>("SELECT id, name, message FROM greetings")
        .fetch_all(pool.inner())
        .await
        .unwrap();
    Json(greetings)
}

#[get("/greetings/view")]
async fn view_greetings(pool: &State<SqlitePool>, tera: &State<Tera>) -> RawHtml<String> {
    let greetings = sqlx::query_as::<_, Greeting>("SELECT id, name, message FROM greetings")
        .fetch_all(pool.inner())
        .await
        .unwrap();
    let mut context = Context::new();
    context.insert("greetings", &greetings);
    let rendered = tera.render("greetings.html", &context).unwrap();
    RawHtml(rendered)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Create data directory if it doesn't exist
    std::fs::create_dir_all("data").expect("Failed to create data directory");
    
    // Connect to database with better path
    let database_url = "sqlite://data/greeting.db";
    let pool = SqlitePool::connect(database_url)
        .await
        .expect("Failed to connect to database");
    
    // Create table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS greetings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            message TEXT NOT NULL
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");
    
    let tera = Tera::new("templates/**/*")
        .expect("Failed to initialize template engine");

    println!("🚀 Starting server at http://localhost:8000");

    rocket::build()
        .manage(pool)
        .manage(tera)
        .mount("/", routes![hello_world, hello_name, create_greeting, get_greetings, view_greetings])
        .launch()
        .await
        .map(|_| ())
}


