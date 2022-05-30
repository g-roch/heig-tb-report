#![feature(proc_macro_hygiene, decl_macro)]

use std::sync::Mutex;

#[macro_use]
extern crate rocket;

use lazy_static::lazy_static;
use rocket::config::{Config, Environment};
use rocket::request::Form;
use rocket::response::content;
use rocket_contrib::json::Json;

use lib::Bulletin;
use lib::BulletinBoard;
use lib::VoteOption;
use lib::BOARD_PORT;

lazy_static! {
    static ref BOARD: Mutex<BulletinBoard> = Default::default();
    static ref OPTIONS: Mutex<Vec<VoteOption>> = Default::default();
}

#[derive(FromForm)]
struct AddOption {
    text: String,
}

#[get("/")]
fn index() -> content::Html<&'static str> {
    content::Html(
        r#"<html>
        <b>Ajoute une option de vote</b><br />
        <form method="post" action="/add-option">
            <input name="text" placeholder="Option à ajouter" /><input type="submit" />
        </form>
        <hr/>
        <b><a href="options">Liste des options de votes</a></b>
        <hr />
        <b><a href="board">Listes des bulletins</a></b>
        </html>
        "#,
    )
}

#[post("/add-option", data = "<form>")]
fn add_option(form: Form<AddOption>) -> Json<bool> {
    let mut options = OPTIONS.lock().unwrap();
    if options.iter().filter(|e| e.name == form.text).count() != 0 {
        Json(false)
    } else {
        let id = options.len();
        options.push(VoteOption::new(id, &form.text));
        Json(true)
    }
}

#[get("/options")]
fn options() -> Json<Vec<VoteOption>> {
    let options = OPTIONS.lock().unwrap();
    Json(options.clone())
}

#[get("/board")]
fn board() -> Json<BulletinBoard> {
    let board = BOARD.lock().unwrap();
    Json(board.clone())
}

#[get("/<name>/<value>")]
fn set(name: String, value: String) -> Json<(bool, BulletinBoard)> {
    let mut db = BOARD.lock().unwrap();
    //db.push(Default::default());
    if let Some(entry) = db.get_mut(&name) {
        entry.ballot = format!("{}/{}", entry.ballot, value);
    } else {
        db.insert(
            name.clone(),
            Bulletin {
                user: name,
                ballot: value,
            },
        );
    }

    Json((true, db.clone()))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::build(Environment::active()?)
        .port(BOARD_PORT)
        .finalize()?;
    let launcherror = rocket::custom(config)
        .mount("/", routes![index, add_option, options, board])
        .launch();
    Err(launcherror)?
}
