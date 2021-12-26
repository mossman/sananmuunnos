#[macro_use] extern crate rocket;

mod lib;

use lib::{SpoonMaps, WordResult};
use rocket::State;
use rocket::serde::json::Json;
use rocket::fs::{FileServer, NamedFile};
use std::env;

use async_std::path::Path;


struct MyConfig {
    spoonmaps: SpoonMaps
}

#[get("/word/<word>")]
async fn word(word: &str, state: &State<MyConfig>) -> Json<Vec<WordResult>> {
    let results = state.spoonmaps.spoonerism(word);
    Json(results)
}

#[get("/")]
async fn root() -> Option<NamedFile> {
    NamedFile::open(Path::new(&env::var("STATIC_DIR").unwrap()).join("index.html")).await.ok()
}

#[launch]
fn rocket() -> _ {
    let spoonmaps = SpoonMaps::from_kotus_xml("sanat.xml");
    let state = MyConfig { spoonmaps: spoonmaps };

    rocket::build()
        .mount("/api", routes![word])
        .mount("/static", FileServer::from(env::var("STATIC_DIR").unwrap()))
        .mount("/", routes![root])
        .manage(state)
}
