#[macro_use]
extern crate rocket;

use annotation::{Annotate, ClosestGene, GeneAnnotation};
use csv::WriterBuilder;
use dna::{self, Format, Location, RepeatMask, DNA};
use loctogene::{self, GenomicFeature, Level, Loctogene, TSSRegion};
use rocket::{
    http::ContentType,
    response::status::BadRequest,
    serde::{json::Json, Serialize},
};
use serde::Deserialize;
use std::{env::consts::ARCH, error::Error};
use utils::{
    create_genesdb, parse_assembly_from_route, parse_bool, parse_closest_n_from_route,
    parse_level_from_route, parse_loc_from_route, parse_tss_from_query,
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
pub struct GeneAnnotationTable {
    pub location: Vec<Location>,
    pub gene_ids: Vec<String>,
    pub gene_symbols: Vec<String>,
    pub prom_labels: Vec<String>,
    pub tss_dists: Vec<String>,
    pub closest_genes: Vec<Vec<ClosestGene>>,
}
#[derive(Serialize)]
pub struct AnnotationJsonData {
    pub headers: Vec<String>,
    pub data: GeneAnnotationTable,
}

#[derive(Serialize)]
pub struct AnnotationJsonResp {
    pub data: GeneAnnotationTable,
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

fn make_table() -> Result<String, Box<dyn Error>> {
    let mut wtr = WriterBuilder::new().delimiter(b'\t').from_writer(vec![]);

    // Since we're writing records manually, we must explicitly write our
    // header record. A header record is written the same way that other
    // records are written.
    wtr.write_record(&vec!["City", "State", "Population", "Latitude", "Longitude"])?;
     wtr.write_record(&[
        "Davidsons Landing",
        "AK",
        "",
        "65.2419444",
        "-165.2716667"
    ])?;
     wtr.write_record(&["Kenai", "AK", "7610", "60.5544444", "-151.2583333"])?;
     wtr.write_record(&["Oakman", "AL", "", "33.7133333", "-87.3886111"])?;

    let inner: Vec<u8> =  wtr.into_inner()?;
    let data: String =  String::from_utf8(inner)?;

    Ok(data)
}

#[get("/cheese")]
fn cheese_route() -> Result<(ContentType, String), BadRequest<Json<MessageResp>>> {

    let data: String = unwrap_bad_req!(make_table());

    Ok((ContentType::Text, data))
}

#[get("/?<chr>&<start>&<end>&<assembly>&<format>&<mask>&<rev>&<comp>")]
fn dna_route(
    chr: Option<&str>,
    start: Option<i32>,
    end: Option<i32>,
    assembly: Option<&str>,
    rev: Option<&str>,
    comp: Option<&str>,
    format: Option<&str>,
    mask: Option<&str>,
) -> Result<Json<DNAJsonResp>, BadRequest<Json<MessageResp>>> {
    let loc: dna::Location = unwrap_bad_req!(utils::parse_loc_from_route(
        chr, start, end, "chr1", 100000, 100100
    ));

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

    let dna_db: DNA = DNA::new(&format!("data/dna/{}", a));

    let dna: String = unwrap_bad_req!(dna_db.get_dna(&loc, r, rc, &format, &repeat_mask));

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
    start: Option<i32>,
    end: Option<i32>,
    assembly: Option<&str>,
    level: Option<&str>,
) -> Result<Json<GenesJsonResp>, BadRequest<Json<MessageResp>>> {
    let location: dna::Location = unwrap_bad_req!(parse_loc_from_route(
        chr, start, end, "chr3", 187721381, 187745468
    ));

    let a: String = parse_assembly_from_route(assembly);

    let l: Level = parse_level_from_route(level);

    let genesdb: Loctogene = unwrap_bad_req!(create_genesdb(&a));

    let features: Vec<GenomicFeature> = unwrap_bad_req!(genesdb.get_genes_within(&location, l));

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
    start: Option<i32>,
    end: Option<i32>,
    assembly: Option<&str>,
    n: Option<u16>,
    level: Option<&str>,
) -> Result<Json<GenesJsonResp>, BadRequest<Json<MessageResp>>> {
    let location: Location = unwrap_bad_req!(parse_loc_from_route(
        chr, start, end, "chr3", 187721381, 187745468
    ));

    let a: String = parse_assembly_from_route(assembly);

    let closest_n: u16 = parse_closest_n_from_route(n);

    let l: loctogene::Level = parse_level_from_route(level);

    let genesdb: Loctogene = unwrap_bad_req!(create_genesdb(&a));

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

#[derive(Serialize, Deserialize, Debug)]
struct AnnotationBody {
    locations: Vec<Location>,
}

#[post("/?<assembly>&<n>&<tss>", data = "<body>")]
fn annotation_route(
    assembly: Option<&str>,
    n: Option<u16>,
    tss: Option<&str>,
    body: Json<AnnotationBody>,
) -> Result<Json<AnnotationJsonResp>, BadRequest<Json<MessageResp>>> {
    let a: String = parse_assembly_from_route(assembly);

    let closest_n: u16 = parse_closest_n_from_route(n);

    let ts: TSSRegion = parse_tss_from_query(tss);

    println!("{:?}", body);
    // let location: dna::Location =
    //     match parse_loc_from_route(chr, start, end, "chr3", 187721381, 187745468) {
    //         Ok(loc) => loc,
    //         Err(err) => return Err(BadRequest(Json(MessageResp { message: err }))),
    //     };

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

    let genesdb: Loctogene = unwrap_bad_req!(create_genesdb(&a));

    let annotatedb: Annotate = Annotate::new(genesdb, ts, closest_n);

    let l = body.locations.len();

    // let mut headers: Vec<String> = Vec::with_capacity(6 + l);

    // headers.push("Location".to_owned());
    // headers.push("ID".to_owned());
    // headers.push("Gene Symbol".to_owned());
    // headers.push(format!(
    //     "Relative To Gene (prom=-{}/+{}kb)",
    //     ts.offset_5p() / 1000,
    //     ts.offset_3p() / 1000
    // ));
    // headers.push("TSS Distance".to_owned());

    // for i in 1..(l + 1) {
    //     headers.push(format!("#{} Closest ID", i));
    //     headers.push(format!("#{} Closest Gene Symbols", i));
    //     headers.push(format!(
    //         "#{} Relative To Closet Gene (prom=-{}/+{}kb)",
    //         i,
    //         ts.offset_5p() / 1000,
    //         ts.offset_3p() / 1000
    //     ));
    //     headers.push(format!("#{} TSS Closest Distance", i));
    // }

    let mut table: GeneAnnotationTable = GeneAnnotationTable {
        location: Vec::with_capacity(l),
        gene_ids: Vec::with_capacity(l),
        gene_symbols: Vec::with_capacity(l),
        prom_labels: Vec::with_capacity(l),
        tss_dists: Vec::with_capacity(l),
        closest_genes: Vec::with_capacity(closest_n as usize),
    };

    // create one col for each closest gene
    while table.closest_genes.len() < l {
        table.closest_genes.push(Vec::with_capacity(l));
    }

    for location in body.locations.iter() {
        let annotation: GeneAnnotation = unwrap_bad_req!(annotatedb.annotate(&location));

        table.location.push(location.clone());
        table.gene_ids.push(annotation.gene_ids);
        table.gene_ids.push(annotation.gene_symbols);
        table.prom_labels.push(annotation.prom_labels);
        table.tss_dists.push(annotation.tss_dists);

        for (i, closest_gene) in annotation.closest_genes.iter().enumerate() {
            table
                .closest_genes
                .get_mut(i)
                .unwrap()
                .push(closest_gene.clone());
        }
    }

    Ok(Json(AnnotationJsonResp { data: table }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![about_route, cheese_route])
        .mount("/v1/dna", routes![dna_route])
        .mount(
            "/v1/genes",
            routes![within_genes_route, closest_genes_route],
        )
        .mount("/v1/annotation", routes![annotation_route])
}
