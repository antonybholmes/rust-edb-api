#[macro_use]
extern crate rocket;

use dotenvy::dotenv;

use dna::{DnaDb, Format, Location, RepeatMask};
use genes::{
    annotate::Annotate,
    loctogene::{GenomicFeature, Level, LoctogeneDb, TSSRegion},
};
use io::{
    dna::{DnaJsonResp, DnaResp},
    genes::{
        create_genesdb, parse_level_from_route, parse_tss_from_query, GenesJsonData, GenesJsonResp, LocationGenes
    },
};
use rocket::{
    http::ContentType,
    serde::{json::Json, Serialize},
};

use std::env::consts::ARCH;

use utils::{
    create_userdb, parse_bool, parse_closest_n_from_route, parse_output_from_query, unwrap_bad_req,
    ErrorResp, JsonResult,
};

use auth::{
    jwt::{create_jwt, JWTResp, JWT},
    AuthError, AuthUser, LoginUser, UserDb,
};

use io::dna::DnaBody;

mod io;
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

pub fn register(user: &LoginUser) -> Result<String, AuthError> {
    let userdb: UserDb = create_userdb()?;

    let auth_user: AuthUser = userdb.create_user(user)?;

    let jwt: String = create_jwt(&auth_user)?;

    Ok(jwt)
}

#[post("/register", format = "application/json", data = "<user>")]
pub fn register_route(user: Json<LoginUser>) -> JsonResult<JWTResp> {
    let jwt: String = unwrap_bad_req(register(&user))?;

    Ok(Json(JWTResp { jwt }))
}

pub fn login(user: &LoginUser) -> Result<String, AuthError> {
    let userdb: UserDb = create_userdb()?;

    let auth_user: AuthUser = userdb.find_user_by_email(user)?;

    let jwt: String = create_jwt(&auth_user)?;

    Ok(jwt)
}

#[post("/login", format = "application/json", data = "<user>")]
pub fn login_route(user: Json<LoginUser>) -> JsonResult<JWTResp> {
    let jwt: String = unwrap_bad_req(login(&user))?;

    Ok(Json(JWTResp { jwt }))
}

#[post(
    "/<assembly>?<format>&<mask>&<rev>&<comp>",
    format = "application/json",
    data = "<data>"
)]
fn dna_route(
    assembly: &str,
    rev: Option<&str>,
    comp: Option<&str>,
    format: Option<&str>,
    mask: Option<&str>,
    data: Json<DnaBody>,
    jwt: Result<JWT, AuthError>,
) -> JsonResult<DnaJsonResp> {
    // test if key valid
    let _key: JWT = unwrap_bad_req(jwt)?;

    let loc: &Location = data.locations.get(0).unwrap();

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

    let dna_db: DnaDb = DnaDb::new(&format!("data/dna/{}", assembly));

    let mut seqs: Vec<dna::DNA> = Vec::with_capacity(data.locations.len());

    for location in data.locations.iter() {
        let dna: String = unwrap_bad_req(dna_db.dna(&loc, r, rc, &format, &repeat_mask))?;

        seqs.push(dna::DNA {
            location: location.clone(),
            dna,
        })
    }

    Ok(Json(DnaJsonResp {
        data: DnaResp {
            assembly: assembly.to_string(),
            seqs,
        },
    }))
}

#[post(
    "/within/<assembly>?<level>",
    format = "application/json",
    data = "<data>"
)]
fn within_genes_route(
    assembly: &str,
    level: Option<&str>,
    data: Json<io::dna::DnaBody>,
    jwt: Result<JWT, AuthError>,
) -> JsonResult<io::genes::GenesJsonResp> {
    let _key: JWT = unwrap_bad_req(jwt)?;

    let l: Level = parse_level_from_route(level);

    let genesdb: LoctogeneDb = unwrap_bad_req(create_genesdb(assembly))?;

    let mut all_features: Vec<LocationGenes> = Vec::with_capacity(data.locations.len());

    for location in data.locations.iter() {
        let features: Vec<GenomicFeature> =
            unwrap_bad_req(genesdb.get_genes_within(&location, &l))?;

        all_features.push(LocationGenes {
            location: location.clone(),
            features,
        })
    }

    Ok(Json(GenesJsonResp {
        data: GenesJsonData {
            level: l,
            features: all_features,
        },
    }))
}

#[post(
    "/closest/<assembly>?<n>&<level>",
    format = "application/json",
    data = "<data>"
)]
fn closest_genes_route(
    assembly: &str,
    n: Option<u16>,
    level: Option<&str>,
    data: Json<DnaBody>,
    jwt: Result<JWT, AuthError>,
) -> JsonResult<GenesJsonResp> {
    let _key: JWT = unwrap_bad_req(jwt)?;

    let closest_n: u16 = parse_closest_n_from_route(n);

    let l: Level = parse_level_from_route(level);

    let genesdb: LoctogeneDb = unwrap_bad_req(create_genesdb(assembly))?;

    let mut all_features: Vec<LocationGenes> = Vec::with_capacity(data.locations.len());

    for location in data.locations.iter() {
        let features: Vec<GenomicFeature> =
            unwrap_bad_req(genesdb.get_closest_genes(&location, closest_n, l))?;

        all_features.push(LocationGenes {
            location: location.clone(),
            features,
        })
    }

    Ok(Json(GenesJsonResp {
        data: GenesJsonData {
            level: l,
            features: all_features,
        },
    }))
}

#[post("/<assembly>?<n>&<tss>&<output>", data = "<body>")]
fn annotation_route(
    assembly: &str,
    n: Option<u16>,
    tss: Option<&str>,
    output: Option<&str>,
    body: Json<DnaBody>,
) -> Result<(ContentType, String), ErrorResp> {
    //let a: String = parse_assembly_from_route(assembly);

    let closest_n: u16 = parse_closest_n_from_route(n);

    let ts: TSSRegion = parse_tss_from_query(tss);

    let output: String = parse_output_from_query(output);

    let genesdb = unwrap_bad_req(create_genesdb(assembly))?;

    let annotatedb = Annotate::new(genesdb, ts, closest_n);

    let d: String = unwrap_bad_req(if output == "text" {
        io::genes::make_gene_table(&annotatedb, &body, closest_n, &ts)
    } else {
        io::genes::make_gene_json(&annotatedb, &body, closest_n)
    })?;

    let content_type: ContentType = if output == "text" {
        ContentType::Text
    } else {
        ContentType::JSON
    };

    Ok((content_type, d))
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .mount("/", routes![about_route, register_route, login_route])
        .mount("/auth/dna", routes![dna_route])
        .mount(
            "/auth/genes",
            routes![within_genes_route, closest_genes_route],
        )
        .mount("/auth/annotation", routes![annotation_route])
}
