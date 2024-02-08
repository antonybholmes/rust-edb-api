#[macro_use]
extern crate rocket;

use annotation::Annotate;

use dna::{Format, Location, RepeatMask, DNA};
use loctogene::{self, GenomicFeature, Level, Loctogene, TSSRegion};
use rocket::{
    http::ContentType,
    response::status::BadRequest,
    serde::{json::Json, Serialize},
};

use std::env::consts::ARCH;
use utils::{
    create_genesdb,
    genes::{GenesJsonData, GenesJsonResp},
    parse_bool, parse_closest_n_from_route, parse_format_from_query, parse_level_from_route,
    parse_loc_from_route, parse_tss_from_query, AnnotationBody, DNAJsonData, DNAJsonResp,
};

mod tests;
mod utils;

macro_rules! unwrap_bad_req {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(err) => {
                return Err(BadRequest(Json(MessageResp {
                    message: err.to_string(),
                })))
            }
        }
    };
}

const NAME: &'static str = "edb-api";
const VERSION: &'static str = "1.0.0";
const COPYRIGHT: &'static str = "Copyright (C) 2024 Antony Holmes";

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

#[get("/about")]
fn about_route() -> Json<AboutJsonResp> {
    Json(AboutJsonResp {
        name: NAME,
        version: VERSION,
        copyright: COPYRIGHT,
        arch: ARCH,
    })
}

#[get("/<assembly>?<chr>&<start>&<end>&<format>&<mask>&<rev>&<comp>")]
fn dna_route(
    assembly: &str,
    chr: Option<&str>,
    start: Option<i32>,
    end: Option<i32>,
    rev: Option<&str>,
    comp: Option<&str>,
    format: Option<&str>,
    mask: Option<&str>,
) -> Result<Json<DNAJsonResp>, BadRequest<Json<MessageResp>>> {
    let loc: dna::Location = unwrap_bad_req!(utils::parse_loc_from_route(
        chr, start, end, "chr1", 100000, 100100
    ));

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
            "lower" => Format::Lower,
            "upper" => Format::Upper,
            _ => Format::None,
        },
        None => Format::None,
    };

    let repeat_mask: RepeatMask = match mask {
        Some(rc) => match rc {
            "lower" => RepeatMask::Lower,
            "n" => RepeatMask::N,
            _ => RepeatMask::None,
        },
        None => RepeatMask::None,
    };

    let dna_db: DNA = DNA::new(&format!("data/dna/{}", assembly));

    let dna: String = unwrap_bad_req!(dna_db.get_dna(&loc, r, rc, &format, &repeat_mask));

    Ok(Json(DNAJsonResp {
        data: DNAJsonData {
            location: loc.to_string(),
            dna,
        },
    }))
}

#[get("/within/<assembly>?<chr>&<start>&<end>&<level>")]
fn within_genes_route(
    assembly: &str,
    chr: Option<&str>,
    start: Option<i32>,
    end: Option<i32>,
    level: Option<&str>,
) -> Result<Json<GenesJsonResp>, BadRequest<Json<MessageResp>>> {
    let location: dna::Location = unwrap_bad_req!(parse_loc_from_route(
        chr, start, end, "chr3", 187721381, 187745468
    ));

    let l: Level = parse_level_from_route(level);

    let genesdb: Loctogene = unwrap_bad_req!(create_genesdb(assembly));

    let features: Vec<GenomicFeature> = unwrap_bad_req!(genesdb.get_genes_within(&location, l));

    Ok(Json(GenesJsonResp {
        data: GenesJsonData {
            location,
            level: l,
            features,
        },
    }))
}

#[get("/closest/<assembly>?<chr>&<start>&<end>&<n>&<level>")]
fn closest_genes_route(
    assembly: &str,
    chr: Option<&str>,
    start: Option<i32>,
    end: Option<i32>,
    n: Option<u16>,
    level: Option<&str>,
) -> Result<Json<GenesJsonResp>, BadRequest<Json<MessageResp>>> {
    let location: Location = unwrap_bad_req!(parse_loc_from_route(
        chr, start, end, "chr3", 187721381, 187745468
    ));

    let closest_n: u16 = parse_closest_n_from_route(n);

    let l: loctogene::Level = parse_level_from_route(level);

    let genesdb: Loctogene = unwrap_bad_req!(create_genesdb(assembly));

    let features: Vec<GenomicFeature> =
        unwrap_bad_req!(genesdb.get_closest_genes(&location, closest_n, l));

    Ok(Json(GenesJsonResp {
        data: GenesJsonData {
            location,
            level: l,
            features,
        },
    }))
}

#[post("/<assembly>?<n>&<tss>&<format>", data = "<body>")]
fn annotation_route(
    assembly: &str,
    n: Option<u16>,
    tss: Option<&str>,
    format: Option<&str>,
    body: Json<AnnotationBody>,
) -> Result<(ContentType, String), BadRequest<Json<MessageResp>>> {
    //let a: String = parse_assembly_from_route(assembly);

    let closest_n: u16 = parse_closest_n_from_route(n);

    let ts: TSSRegion = parse_tss_from_query(tss);

    let format: String = parse_format_from_query(format);

    let genesdb: Loctogene = unwrap_bad_req!(create_genesdb(assembly));

    let annotatedb: Annotate = Annotate::new(genesdb, ts, closest_n);

    let d: String = unwrap_bad_req!(if format == "text" {
        utils::genes::make_gene_table(&annotatedb, &body, closest_n, &ts)
    } else {
        utils::genes::make_gene_json(&annotatedb, &body, closest_n)
    });

    let content_type: ContentType = if format == "text" {
        ContentType::Text
    } else {
        ContentType::JSON
    };

    Ok((content_type, d))
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
