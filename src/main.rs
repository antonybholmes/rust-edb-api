#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Serialize};

use tracing::level_filters::LevelFilter;
use tracing_appender::rolling;
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, layer::SubscriberExt, Layer, Registry};

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
    log::debug!("hmm");

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

    //
    // start: configure logging
    //

    let _ = LogTracer::init(); //LogTracer::init_with_filter(log::LevelFilter::Debug);

    let debug_file = rolling::daily("./logs", "debug");

    let subscriber = Registry::default()
        .with(
            fmt::layer()
                .json()
                .with_ansi(false)
                .with_writer(debug_file)
                .with_filter(LevelFilter::DEBUG),
        )
        .with(fmt::layer().with_writer(std::io::stdout));

    tracing::subscriber::set_global_default(subscriber).expect("unable to set global subscriber");

    //
    // end: configure logging
    //

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
