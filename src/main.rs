
#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, json::Json};
 
mod tests;

#[derive(Serialize)]
pub struct Message {
    pub message: String,
}

#[get("/")]
fn index() -> Json<Message> {
    Json(Message{message:"cake".to_string()})
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
