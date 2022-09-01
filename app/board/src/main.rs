#![feature(proc_macro_hygiene, decl_macro)]

use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Mutex;

#[macro_use]
extern crate rocket;

use lazy_static::lazy_static;
use rocket::config::{Config, Environment};
use rocket::request::Form;
use rocket::response::content;
use rocket_contrib::json::Json;

use lib::crypto::{pr1, pr2};
use lib::Bulletin;
use lib::BulletinBoard;
use lib::VoteOption;
use lib::BOARD_PORT;

use curve25519_dalek::ristretto::CompressedRistretto;
use serde_json;

type Round1 = (Vec<CompressedRistretto>, Vec<pr1::CompactProof>);
type Round2 = (Vec<CompressedRistretto>, Vec<pr2::CompactProof>);

lazy_static! {
    static ref BOARD: Mutex<BulletinBoard> = Default::default();
    static ref OPTIONS: Mutex<Vec<VoteOption>> = Default::default();
    static ref STATE: Mutex<u8> = Mutex::new(0);
    static ref VOTERS: Mutex<Vec<Option<Round1>>> = Default::default();
    static ref ROUND2: Mutex<Vec<Option<Round2>>> = Default::default();
}

#[derive(FromForm)]
struct AddOption {
    text: String,
}

#[derive(FromForm)]
struct Round1Form {
    voter: usize,
    data: String,
}

#[derive(FromForm)]
struct Round2Form {
    voter: usize,
    data: String,
}

#[get("/")]
fn index() -> content::Html<String> {
    let options = OPTIONS.lock().unwrap();
    let state = STATE.lock().unwrap();
    let voters = VOTERS.lock().unwrap();
    let options = options
        .iter()
        .map(|opt| format!("{} = {}", opt.id, opt.name))
        .collect::<Vec<_>>()
        .join("<br/>");
    let bulletins;
    let form_add_opt;
    match state.deref() {
        0 => {
            form_add_opt = r#"<b>Ajoute une option de vote</b><br />
                <form method="post" action="/add-option">
                    <input name="text" placeholder="Option à ajouter" /><input type="submit" />
                </form>
                "#;
            bulletins = r#"<a href="/open">Autoriser la publication des clés</a>"#;
        }
        1 => {
            form_add_opt = r#"<meta http-equiv="refresh" content="5" >"#;
            bulletins = r#"<a href="/openvote">Autoriser les votants a se connecter</a>"#;
        }
        2 => {
            form_add_opt = r#"<meta http-equiv="refresh" content="5" >"#;
            bulletins = r#"<a href="/openvote2">Autoriser les votants a voter</a>"#;
        }
        _ => {
            form_add_opt = r#"<meta http-equiv="refresh" content="5" >"#;
            bulletins = r#"<b><a href="board">Listes des bulletins</a></b>"#;
        }
    }
    let voters = format!(
        "{:?}",
        voters
            .iter()
            .map(|i| match i {
                None => None,
                Some((a, b)) => Some(serde_json::to_string(&(a, b)).unwrap()),
            })
            .map(|a| format!("{a:?}"))
            .collect::<Vec<_>>()
            .join("<br/>")
    );
    content::Html(format!(
        r#"<html>
        <h1>State: {state}</h1>
        {form_add_opt}
        <hr/>
        <b>Liste des options de votes <a href="options">(json)</a></b><br />
        {options}
        <hr />
        <b>Votants</b><br />
        {voters}
        <hr />
        {bulletins}
        </html>
        "#
    ))
}

#[get("/get-round1")]
fn get_round1() -> Json<Vec<Round1>> {
    let voters = VOTERS.lock().unwrap();
    let result: Option<Vec<Round1>> = voters.iter().map(|c| c.clone()).collect();
    if result.is_some() {
        let mut state = STATE.lock().unwrap();
        if state.deref() == &2 {
            *state = 3;
        }
    }
    Json(result.unwrap())
}

#[get("/get-round2")]
fn get_round2() -> Json<Option<Vec<Round2>>> {
    let round2 = ROUND2.lock().unwrap();
    let result: Option<Vec<Round2>> = round2.iter().map(|c| c.clone()).collect();
    if result.is_some() {
        let mut state = STATE.lock().unwrap();
        if state.deref() == &3 {
            *state = 4;
        }
    }
    Json(result)
}

#[post("/add-round1", data = "<form>")]
fn add_round1(form: Form<Round1Form>) -> Json<bool> {
    let mut voters = VOTERS.lock().unwrap();
    let round_1: Round1 = serde_json::from_str(&form.data).unwrap();
    let voterid = form.voter;
    voters[voterid] = Some(round_1);
    Json(true)
}

#[post("/add-round2", data = "<form>")]
fn add_round2(form: Form<Round2Form>) -> Json<bool> {
    let mut round2 = ROUND2.lock().unwrap();
    let round_2: Round2 = serde_json::from_str(&form.data).unwrap();
    let voterid = form.voter;
    round2[voterid] = Some(round_2);

    //if voters.iter().all(Option::is_some) {
    //    let mut state = STATE.lock().unwrap();
    //    *(state.deref_mut()) = 2;
    //}
    Json(true)
}
#[post("/add-option", data = "<form>")]
fn add_option(form: Form<AddOption>) -> content::Html<&'static str> {
    let mut options = OPTIONS.lock().unwrap();
    if options.iter().filter(|e| e.name == form.text).count() != 0 {
        content::Html(r#"<html><h1>Error</h1></html>"#)
    } else {
        let id = options.len();
        options.push(VoteOption::new(id, &form.text));
        content::Html(
            r#"<html><head><meta http-equiv="refresh" content="0; url=/" /></head></html>"#,
        )
    }
}

#[get("/open")]
fn open() -> content::Html<&'static str> {
    let mut state = STATE.lock().unwrap();
    *(state.deref_mut()) = 1;
    content::Html(r#"<html><head><meta http-equiv="refresh" content="0; url=/" /></head></html>"#)
}

#[get("/openvote")]
fn openvote() -> content::Html<&'static str> {
    let mut state = STATE.lock().unwrap();
    *(state.deref_mut()) = 2;
    content::Html(r#"<html><head><meta http-equiv="refresh" content="0; url=/" /></head></html>"#)
}

#[get("/openvote2")]
fn openvote2() -> content::Html<&'static str> {
    let mut state = STATE.lock().unwrap();
    *(state.deref_mut()) = 3;
    content::Html(r#"<html><head><meta http-equiv="refresh" content="0; url=/" /></head></html>"#)
}

#[get("/newvoter")]
fn newvoter() -> Json<usize> {
    let mut voters = VOTERS.lock().unwrap();
    let mut round2 = ROUND2.lock().unwrap();
    voters.push(None);
    round2.push(None);
    Json(voters.len() - 1)
}

#[get("/canvote")]
fn canvote() -> Json<bool> {
    let state = STATE.lock().unwrap();
    Json(state.deref() == &2)
}

#[get("/isopen")]
fn isopen() -> Json<bool> {
    let state = STATE.lock().unwrap();
    Json(state.deref() == &1)
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
        .mount(
            "/",
            routes![
                index, add_option, options, board, open, isopen, newvoter, add_round1, canvote,
                openvote, openvote2, get_round1, add_round2, get_round2
            ],
        )
        .launch();
    Err(launcherror)?
}
