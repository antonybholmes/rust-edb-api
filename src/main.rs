#[macro_use]
extern crate rocket;

use std::env::consts::ARCH;

use rocket::{
    response::status::BadRequest,
    serde::{json::Json, Serialize},
};

const NAME: &str = "edb-api";
const VERSION: &str = "1.0.0";
const COPYRIGHT: &str = "Copyright (C) 2024 Antony Holmes";

mod tests;

#[derive(Serialize)]
pub struct MessageResp {
    pub message: String,
}

#[derive(Serialize)]
pub struct AboutJsonResp {
    pub name: String,
    pub version: String,
    pub copyright: String,
    pub arch: String,
}

#[derive(Serialize)]
pub struct DNAJsonResp {
    pub location: String,
    pub dna: String,
}

#[get("/about")]
fn about_route() -> Json<AboutJsonResp> {
    Json(AboutJsonResp {
        name: NAME.to_string(),
        version: VERSION.to_string(),
        copyright: COPYRIGHT.to_string(),
        arch: ARCH.to_string(),
    })
}

fn parse_loc_from_route(
    chr: Option<&str>,
    start: Option<u32>,
    end: Option<u32>,
) -> Result<dna::Location, String> {
    let c = match chr {
        Some(c) => c,
        None => "chr3",
    };

    let s = match start {
        Some(s) => s,
        None => 187721377,
    };

    let e = match end {
        Some(e) => e,
        None => 187721377,
    };

    let loc: dna::Location = match dna::Location::new(c, s, e) {
        Ok(loc) => loc,
        Err(err) => return Err(err),
    };

    Ok(loc)
}

#[get("/?<chr>&<start>&<end>&<rev>&<comp>")]
fn dna_route(
    chr: Option<&str>,
    start: Option<u32>,
    end: Option<u32>,
    rev: Option<bool>,
    comp: Option<bool>,
) -> Result<Json<DNAJsonResp>, BadRequest<Json<MessageResp>>> {
    let loc: dna::Location = match parse_loc_from_route(chr, start, end) {
        Ok(loc) => loc,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    let r = match rev {
        Some(r) => r,
        None => false,
    };

    let rc = match comp {
        Some(rc) => rc,
        None => false,
    };

    let dir: &str = "/ifs/scratch/cancer/Lab_RDF/ngs/dna/hg19";

    let dna_db: dna::DNA = dna::DNA::new(dir);

    let dna: String = match dna_db.get_dna(&loc, r, rc) {
        Ok(dna) => dna,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    return Ok(Json(DNAJsonResp {
        location: loc.to_string(),
        dna,
    }));
}

#[get("/within?<chr>&<start>&<end>&<assembly>")]
fn within_genes_route(
    chr: Option<&str>,
    start: Option<u32>,
    end: Option<u32>,
    assembly: Option<&str>,
) -> Result<Json<loctogene::Features>, BadRequest<Json<MessageResp>>> {
    let loc: dna::Location = match parse_loc_from_route(chr, start, end) {
        Ok(loc) => loc,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    let a = match assembly {
        Some(assembly) => assembly,
        None => "grch38",
    };

    let l: u32 = 1;

    let genesdb: loctogene::Loctogene =
        match loctogene::Loctogene::new(&format!("/data/loctogene/{}.db", a)) {
            Ok(db) => db,
            Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
        };

    let records: loctogene::Features = match genesdb.get_genes_within(&loc, l) {
        Ok(records) => records,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    Ok(Json(records))
    //Err(BadRequest(Json(MessageResp { message: "ckk".to_string() })))
}

#[get("/closest?<chr>&<start>&<end>&<assembly>&<n>")]
fn closest_genes_route(
    chr: Option<&str>,
    start: Option<u32>,
    end: Option<u32>,
    assembly: Option<&str>,
    n: Option<u16>,
) -> Result<Json<loctogene::Features>, BadRequest<Json<MessageResp>>> {
    let loc: dna::Location = match parse_loc_from_route(chr, start, end) {
        Ok(loc) => loc,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    let a = match assembly {
        Some(assembly) => assembly,
        None => "grch38",
    };

    let nn = match n {
        Some(nn) => nn,
        None => 10,
    };

    let l: u32 = 1;

    let genesdb: loctogene::Loctogene =
        match loctogene::Loctogene::new(&format!("/data/loctogene/{}.db", a)) {
            Ok(db) => db,
            Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
        };

    let records: loctogene::Features = match genesdb.get_closest_genes(&loc, nn, l) {
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
