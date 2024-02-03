#[macro_use]
extern crate rocket;

use rocket::{
    http::Status,
    serde::{json::Json, Serialize},
};

mod tests;

#[derive(Serialize)]
pub struct Message {
    pub message: String,
}

#[derive(Serialize)]
pub struct DNA {
    pub location: String,
    pub dna: String,
}

#[get("/about")]
fn about_route() -> Json<Message> {
    Json(Message {
        message: "cake".to_string(),
    })
}

#[get("/")]
fn dna_route() -> Json<DNA> {
    let loc = dna::Location::parse("chr1:100000-100100");

    let dir: &str = "/ifs/scratch/cancer/Lab_RDF/ngs/dna/hg19";

    let dna_db: dna::DNA = dna::DNA::new(dir);

    let dna = dna_db.get_dna(&loc, true, true);

    return Json(DNA {
        location: loc.to_string(),
        dna,
    });
}

#[get("/within")]
fn within_genes_route() -> Json<loctogene::Features> {
    let loc: dna::Location = dna::Location::parse("chr3:187721377-187745725");

    let genesdb: loctogene::Loctogene = match loctogene::Loctogene::new(
        "/home/antony/development/go/docker-go-edb-api/data/loctogene/grch38.db",
    ) {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    let records = match genesdb.get_genes_within(&loc, 1) {
        Ok(records) => records,
        Err(err) => panic!("{}", err),
    };

    return Json(records);
}

#[get("/closest")]
fn closest_genes_route() -> Json<loctogene::Features> {
    let loc: dna::Location = dna::Location::parse("chr3:187721377-187745725");

    let genesdb: loctogene::Loctogene = match loctogene::Loctogene::new(
        "/home/antony/development/go/docker-go-edb-api/data/loctogene/grch38.db",
    ) {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    let records = match genesdb.get_closest_genes(&loc, 10, 1) {
        Ok(records) => records,
        Err(err) => panic!("{}", err),
    };

    return Json(records);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![about_route])
        .mount("/v1/dna", routes![dna_route])
        .mount(
            "/v1/genes",
            routes![within_genes_route, closest_genes_route],
        )
}
