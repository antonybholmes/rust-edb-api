#[macro_use]
extern crate rocket;

use std::env::consts::ARCH;

use rocket::{
    response::status::BadRequest,
    serde::{json::Json, Serialize},
};
use utils::{parse_assembly_from_query, parse_bool, parse_loc_from_route};

const NAME: &'static str = "edb-api";
const VERSION: &'static str = "1.0.0";
const COPYRIGHT: &'static str = "Copyright (C) 2024 Antony Holmes";

mod tests;
mod utils;

#[derive(Serialize)]
pub struct MessageResp {
    pub message: String,
}

#[derive(Serialize)]
pub struct AboutJsonResp {
    pub name: &'static str,
    pub version: &'static str,
    pub copyright: &'static str,
    pub arch: &'static str,
}

#[derive(Serialize)]
pub struct DNAJsonData {
    pub location: String,
    pub dna: String,
}

#[derive(Serialize)]
pub struct DNAJsonResp {
    pub data: DNAJsonData,
}

#[derive(Serialize)]
pub struct GenesJsonResp {
    pub data: loctogene::Features,
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

#[get("/?<chr>&<start>&<end>&<assembly>&<rev>&<comp>")]
fn dna_route(
    chr: Option<&str>,
    start: Option<u32>,
    end: Option<u32>,
    assembly: Option<&str>,
    rev: Option<&str>,
    comp: Option<&str>,
) -> Result<Json<DNAJsonResp>, BadRequest<Json<MessageResp>>> {
    let loc: dna::Location =
        match utils::parse_loc_from_route(chr, start, end, "chr1", 100000, 100100) {
            Ok(loc) => loc,
            Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
        };

    let a: String = parse_assembly_from_query(assembly);

    let r: bool = match rev {
        Some(r) => parse_bool(r),
        None => false,
    };

    let rc: bool = match comp {
        Some(rc) => parse_bool(rc),
        None => false,
    };

    let dna_db: dna::DNA = dna::DNA::new(format!("data/dna/{}", a));

    let dna: String = match dna_db.get_dna(&loc, r, rc) {
        Ok(dna) => dna,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    Ok(Json(DNAJsonResp {
        data: DNAJsonData {
            location: loc.to_string(),
            dna,
        },
    }))
}

#[get("/within?<chr>&<start>&<end>&<assembly>")]
fn within_genes_route(
    chr: Option<&str>,
    start: Option<u32>,
    end: Option<u32>,
    assembly: Option<&str>,
) -> Result<Json<GenesJsonResp>, BadRequest<Json<MessageResp>>> {
    let loc: dna::Location =
        match parse_loc_from_route(chr, start, end, "chr3", 187721381, 187745468) {
            Ok(loc) => loc,
            Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
        };

    let a: String = parse_assembly_from_query(assembly);

    let l: u32 = 1;

    let genesdb: loctogene::Loctogene =
        match loctogene::Loctogene::new(&format!("data/loctogene/{}.db", a)) {
            Ok(db) => db,
            Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
        };

    let records: loctogene::Features = match genesdb.get_genes_within(&loc, l) {
        Ok(records) => records,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    Ok(Json(GenesJsonResp{data:records}))
}

#[get("/closest?<chr>&<start>&<end>&<assembly>&<n>")]
fn closest_genes_route(
    chr: Option<&str>,
    start: Option<u32>,
    end: Option<u32>,
    assembly: Option<&str>,
    n: Option<u16>,
) -> Result<Json<GenesJsonResp>, BadRequest<Json<MessageResp>>> {
    let loc: dna::Location =
        match parse_loc_from_route(chr, start, end, "chr3", 187721381, 187745468) {
            Ok(loc) => loc,
            Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
        };

    let a: &str = match assembly {
        Some(assembly) => assembly,
        None => "grch38",
    };

    let nn: u16 = match n {
        Some(nn) => nn,
        None => 10,
    };

    let l: u32 = 1;

    let genesdb: loctogene::Loctogene =
        match loctogene::Loctogene::new(&format!("data/loctogene/{}.db", a)) {
            Ok(db) => db,
            Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
        };

    let records: loctogene::Features = match genesdb.get_closest_genes(&loc, nn, l) {
        Ok(records) => records,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    Ok(Json(GenesJsonResp{data:records}))
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
