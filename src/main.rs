#[macro_use]
extern crate rocket;

use rocket::{
    response::status::BadRequest,
    serde::{json::Json, Serialize},
};

mod tests;

#[derive(Serialize)]
pub struct MessageResp {
    pub message: String,
}

#[derive(Serialize)]
pub struct DNAJsonResp {
    pub location: String,
    pub dna: String,
}

#[get("/about")]
fn about_route() -> Json<MessageResp> {
    Json(MessageResp {
        message: "cake".to_string(),
    })
}

#[get("/")]
fn dna_route() -> Result<Json<DNAJsonResp>, BadRequest<Json<MessageResp>>> {
    let loc: dna::Location = match dna::Location::parse("chr1:100000-100100") {
        Ok(loc) => loc,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    let dir: &str = "/ifs/scratch/cancer/Lab_RDF/ngs/dna/hg19";

    let dna_db: dna::DNA = dna::DNA::new(dir);

    let dna: String = match dna_db.get_dna(&loc, true, true) {
        Ok(dna) => dna,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    return Ok(Json(DNAJsonResp {
        location: loc.to_string(),
        dna,
    }));
}

#[get("/within")]
fn within_genes_route() -> Result<Json<loctogene::Features>, BadRequest<Json<MessageResp>>>{
    let loc: dna::Location = match dna::Location::parse("chr3:187721377-187745725") {
        Ok(loc) => loc,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    let genesdb: loctogene::Loctogene = match loctogene::Loctogene::new(
        "/home/antony/development/go/docker-go-edb-api/data/loctogene/grch38.db",
    ) {
        Ok(db) => db,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    let records: loctogene::Features = match genesdb.get_genes_within(&loc, 1) {
        Ok(records) => records,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    return Ok(Json(records));
}

#[get("/closest")]
fn closest_genes_route() -> Result<Json<loctogene::Features>, BadRequest<Json<MessageResp>>> {
    let loc: dna::Location = match dna::Location::parse("chr3:187721377-187745725") {
        Ok(loc) => loc,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    let genesdb: loctogene::Loctogene = match loctogene::Loctogene::new(
        "/home/antony/development/go/docker-go-edb-api/data/loctogene/grch38.db",
    ) {
        Ok(db) => db,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    let records: loctogene::Features = match genesdb.get_closest_genes(&loc, 10, 1) {
        Ok(records) => records,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    return Ok(Json(records));
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
