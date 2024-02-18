#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Serialize};

use std::env::consts::ARCH;

mod routes;
mod tests;

 

#[derive(Serialize)]
pub struct AboutJsonResp {
    pub name: String,
    pub version: String,
    pub copyright: String,
}

#[get("/about")]
fn about_route() -> Json<AboutJsonResp> {
    Json(AboutJsonResp {
        name: sys::env::str("NAME"),
        version: sys::env::str("VERSION"),
        copyright: sys::env::str("COPYRIGHT"),
    })
}

#[derive(Serialize)]
pub struct InfoResp {
    pub arch: &'static str,
}

#[get("/info")]
fn info_route() -> Json<InfoResp> {
    Json(InfoResp { arch: ARCH })
}

#[launch]
fn rocket() -> _ {
    sys::env::load();

    sys::env::ls();

    rocket::build()
        .mount("/", routes![about_route, info_route])
        .mount(
            "/users",
            routes![routes::users::register_route, routes::users::login_route],
        )
        .mount("/dna", routes![routes::dna::dna_route])
        .mount(
            "/genes",
            routes![
                routes::genes::within_genes_route,
                routes::genes::closest_genes_route,
                routes::genes::annotation_route
            ],
        )
}
