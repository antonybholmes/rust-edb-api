#[macro_use]
extern crate rocket;

use std::env::consts::ARCH;

use annotation::{Annotate, GeneAnnotation, TSSRegion};
use dna::{self, Format, Location, RepeatMask, DNA};
use loctogene::{self, GenomicFeature, Level, Loctogene};
use rocket::{
    response::status::BadRequest,
    serde::{json::Json, Serialize},
};
use serde::Deserialize;
use utils::{
    create_genesdb, parse_assembly_from_route, parse_bool, parse_closest_n_from_route,
    parse_level_from_route, parse_loc_from_route,
};

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
pub struct GenesJsonData {
    pub location: Location,
    pub level: Level,
    pub features: Vec<GenomicFeature>,
}

#[derive(Serialize)]
pub struct GenesJsonResp {
    pub data: GenesJsonData,
}

#[derive(Serialize)]
pub struct AnnotationJsonData {
    pub location: Location,
    pub level: Level,
    pub annotation: GeneAnnotation,
}

#[derive(Serialize)]
pub struct AnnotationJsonResp {
    pub data: Vec<AnnotationJsonData>,
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

#[get("/?<chr>&<start>&<end>&<assembly>&<format>&<mask>&<rev>&<comp>")]
fn dna_route(
    chr: Option<&str>,
    start: Option<u32>,
    end: Option<u32>,
    assembly: Option<&str>,
    rev: Option<&str>,
    comp: Option<&str>,
    format: Option<&str>,
    mask: Option<&str>,
) -> Result<Json<DNAJsonResp>, BadRequest<Json<MessageResp>>> {
    let loc: dna::Location =
        match utils::parse_loc_from_route(chr, start, end, "chr1", 100000, 100100) {
            Ok(loc) => loc,
            Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
        };

    let a: String = parse_assembly_from_route(assembly);

    let r: bool = match rev {
        Some(r) => parse_bool(r),
        None => false,
    };

    let rc: bool = match comp {
        Some(rc) => parse_bool(rc),
        None => false,
    };

    let format: Format = match format {
        Some(rc) => match rc {
            "lower" => dna::Format::Lower,
            "upper" => dna::Format::Upper,
            _ => dna::Format::None,
        },
        None => dna::Format::None,
    };

    let repeat_mask: RepeatMask = match mask {
        Some(rc) => match rc {
            "lower" => dna::RepeatMask::Lower,
            "n" => dna::RepeatMask::N,
            _ => dna::RepeatMask::None,
        },
        None => dna::RepeatMask::None,
    };

    let dna_db: DNA = DNA::new(format!("data/dna/{}", a));

    let dna: String = match dna_db.get_dna(&loc, r, rc, &format, &repeat_mask) {
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

#[get("/within?<chr>&<start>&<end>&<assembly>&<level>")]
fn within_genes_route(
    chr: Option<&str>,
    start: Option<u32>,
    end: Option<u32>,
    assembly: Option<&str>,
    level: Option<&str>,
) -> Result<Json<GenesJsonResp>, BadRequest<Json<MessageResp>>> {
    let location: dna::Location =
        match parse_loc_from_route(chr, start, end, "chr3", 187721381, 187745468) {
            Ok(loc) => loc,
            Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
        };

    let a: String = parse_assembly_from_route(assembly);

    let l: Level = parse_level_from_route(level);

    let genesdb: Loctogene = match create_genesdb(&a) {
        Ok(db) => db,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    let features: Vec<GenomicFeature> = match genesdb.get_genes_within(&location, l) {
        Ok(records) => records,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    Ok(Json(GenesJsonResp {
        data: GenesJsonData {
            location,
            level: l,
            features,
        },
    }))
}

#[get("/closest?<chr>&<start>&<end>&<assembly>&<n>&<level>")]
fn closest_genes_route(
    chr: Option<&str>,
    start: Option<u32>,
    end: Option<u32>,
    assembly: Option<&str>,
    n: Option<u16>,
    level: Option<&str>,
) -> Result<Json<GenesJsonResp>, BadRequest<Json<MessageResp>>> {
    let location: Location =
        match parse_loc_from_route(chr, start, end, "chr3", 187721381, 187745468) {
            Ok(loc) => loc,
            Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
        };

    let a: String = parse_assembly_from_route(assembly);

    let closest_n: u16 = parse_closest_n_from_route(n);

    let l: loctogene::Level = parse_level_from_route(level);

    let genesdb: Loctogene = match create_genesdb(&a) {
        Ok(db) => db,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    let features: Vec<GenomicFeature> = match genesdb.get_closest_genes(&location, closest_n, l) {
        Ok(records) => records,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    Ok(Json(GenesJsonResp {
        data: GenesJsonData {
            location,
            level: l,
            features,
        },
    }))
}

#[derive(Serialize, Deserialize, Debug)]
struct AnnotationBody<'a> {
    locations: Vec<Location>,
    assembly: &'a str,
    n: u16,
    level: &'a str,
    tss: [i32; 2],
}

#[post("/", data = "<body>")]
fn annotation_route(
    body: Json<AnnotationBody<'_>>,
) -> Result<Json<AnnotationJsonResp>, BadRequest<Json<MessageResp>>> {
    // Print, write to a file, or send to an HTTP server.
    println!("{:?}", body);
    // let location: dna::Location =
    //     match parse_loc_from_route(chr, start, end, "chr3", 187721381, 187745468) {
    //         Ok(loc) => loc,
    //         Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    //     };

    let l: Level = loctogene::Level::from(body.level);

    // let ts: TSSRegion = match tss {
    //     Some(ts) => {
    //         let tokens: Vec<&str> = ts.split(",").collect();

    //         let s: i32 = match tokens[0].parse::<i32>() {
    //             Ok(s) => s,
    //             Err(_) => DEFAULT_TSS_REGION.offset_5p,
    //         };

    //         let e: i32 = match tokens[1].parse::<i32>() {
    //             Ok(s) => s,
    //             Err(_) => DEFAULT_TSS_REGION.offset_3p,
    //         };

    //         TSSRegion::new(s, e)
    //     }
    //     None => DEFAULT_TSS_REGION,
    // };

    let ts: TSSRegion = TSSRegion {
        offset_5p: body.tss[0],
        offset_3p: body.tss[1],
    };

    let genesdb: Loctogene = match create_genesdb(body.assembly) {
        Ok(db) => db,
        Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    };

    let annotatedb: Annotate = Annotate::new(genesdb, ts, body.n);

    let mut data: Vec<AnnotationJsonData> = Vec::with_capacity(body.locations.len());

    for location in body.locations.iter() {
        let annotation: GeneAnnotation = match annotatedb.annotate(&location) {
            Ok(annotation) => annotation,
            Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
        };

        data.push(AnnotationJsonData {
            location: location.clone(),
            level: l,
            annotation,
        });
    }

    Ok(Json(AnnotationJsonResp { data }))
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
        .mount("/v1/annotation", routes![annotation_route])
}
