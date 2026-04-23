//! Proxy server to forward control commands.

use remotectl_common::Command;
use rocket::log::private::info;
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket::{State, get, launch, post, routes};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![set_cmd, get_cmd])
        .manage(Mutex::new(Command::Stop))
}

#[post("/", data = "<command>")]
async fn set_cmd(state: &State<Mutex<Command>>, command: Json<Command>) {
    let command = command.into_inner();
    *state.lock().await = command;
    info!("Set command to: {command:?}");
}

#[get("/")]
async fn get_cmd(state: &State<Mutex<Command>>) -> Json<Command> {
    let command = *state.lock().await;
    info!("Returned command: {command:?}");
    Json(*state.lock().await)
}
