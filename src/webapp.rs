#[macro_use] extern crate rocket;

use sananmuunnos::{SpoonMaps, WordResult};
use rocket::{State};
use rocket::request::{self, Request, FromRequest};
use rocket::fs::{FileServer, NamedFile};
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::http::{Cookie};
use rocket::response::{Debug, status::Created};
use rocket::outcome::Outcome::Success;
use rocket::response::status::Conflict;
use sananmuunnos::schema::{likes,likes_count};
use chrono::Utc;
use std::env;

use rand::{distributions::Alphanumeric, Rng}; // 0.8

use rocket_sync_db_pools::{diesel,database};
use rocket_sync_db_pools::diesel::RunQueryDsl;
use async_std::path::Path;

use sananmuunnos::models::{LikeModel, LikesCount};

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

struct MyConfig {
    spoonmaps: SpoonMaps
}


#[database("sananmuunnos_db")]
struct MyDatabase(diesel::PgConnection);

#[derive(Deserialize, Serialize, Clone)]
struct Like {
    first: String,
    second: String
}

struct Session(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Session, Self::Error> {
        let jar = request.cookies();
        let session = jar.get("session").map(|cookie| cookie.value());
        let mut new_session = String::new();
    
        let session = match session {
            Some(cookie) => String::from(cookie),
            None => {
                let new_rand: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(20)
                    .map(char::from)
                    .collect();
                new_session.push_str(&new_rand);
                jar.add(Cookie::new("session", new_rand));
                new_session
            }
        };
    
        Success(Session(session))
    }
}

#[get("/likes")]
async fn likes_api(db: MyDatabase) -> Result<Json<Vec<LikesCount>>> {
    let items: Vec<LikesCount> = db.run(move |conn| {
        likes_count::table
            .load(conn)
    }).await?;

    Ok(Json(items))
}

#[post("/like", format="application/json", data="<input>")]
async fn like(
    session: Session,
    state: &State<MyConfig>, 
    db: MyDatabase,
    input: Json<Like>
) -> Result<Created<Json<Like>>, Conflict<String>> {

    let valid = state.spoonmaps.check(&input.first, &input.second);
    if !valid {
        Err(Conflict(Some("No such thing".to_string())))
    } else {
        let new_input = input.clone();
        let timestamp = Utc::now().naive_utc();
        let new_like = LikeModel {
            first: new_input.first.clone(),
            second: new_input.second.clone(),
            cookie: session.0,
            timestamp: timestamp
        };
        let inserted = db.run(move |conn| {
            diesel::insert_into(likes::table)
            .values(new_like)
            .execute(conn)
        }).await;

        match inserted {
            Err(e) => {
                Err(Conflict(Some(e.to_string())))
            },
            Ok(0) => {
                Err(Conflict(Some("failed".to_string())))

            },
            _ => Ok(Created::new("/").body(input))

        }
    }
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
        .mount("/api", routes![word, like, likes_api])
        .mount("/static", FileServer::from(env::var("STATIC_DIR").unwrap()))
        .mount("/", routes![root])
        .manage(state)
        .attach(MyDatabase::fairing())
}
