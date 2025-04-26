#[rocket::main]
pub async fn start_server() {
    println!("🌐 Starting web server at http://localhost:8000");
    rocket::build()
        .mount("/", rocket::routes![index])
        .launch()
        .await
        .unwrap();
}

#[rocket::get("/")]
fn index() -> &'static str {
    "Welcome to git-ai Web UI! (More coming soon 🚀)"
}
