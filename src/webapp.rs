#[macro_use] extern crate rocket;

mod lib;

use lib::{SpoonMaps, WordResult};
use rocket::State;
use rocket::serde::json::Json;

struct MyConfig {
    spoonmaps: SpoonMaps
}

#[get("/api/word/<word>")]
fn word(word: &str, state: &State<MyConfig>) -> Json<Vec<WordResult>> {
    let results = state.spoonmaps.spoonerism(word);
    Json(results)
}

#[launch]
fn rocket() -> _ {
    let spoonmaps = SpoonMaps::from_kotus_xml("sanat.xml");
    let state = MyConfig { spoonmaps: spoonmaps };

    rocket::build().mount("/", routes![word]).manage(state)
}
