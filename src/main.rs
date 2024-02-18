#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Serialize};

use std::env::consts::ARCH;

mod routes;
mod tests;
mod utils;

const NAME: &'static str = "edb-api";
const VERSION: &'static str = "1.0.0";
const COPYRIGHT: &'static str = "Copyright (C) 2024 Antony Holmes";

#[derive(Serialize)]
pub struct AboutJsonResp {
    pub name: &'static str,
    pub version: &'static str,
    pub copyright: &'static str,
    pub arch: &'static str,
}

#[get("/about")]
fn about_route() -> Json<AboutJsonResp> {
    Json(AboutJsonResp {
        name: NAME,
        version: VERSION,
        copyright: COPYRIGHT,
        arch: ARCH,
    })
}

#[launch]
fn rocket() -> _ {
    sys::env::load();

    sys::env::ls();

    rocket::build()
        .mount("/", routes![about_route])
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
