pub mod genes;

use std::{error::Error, fmt::Display};

use auth::{AuthResult, UserDb};
use loctogene::{Level, Loctogene, TSSRegion};

use rocket::{response::status::BadRequest, serde::json::Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct MessageResp {
    pub message: String,
}

pub type ErrorResp = BadRequest<Json<MessageResp>>;

// impl From<E> for ErrorResp where E:Display {
//     fn from(err: io::Error) -> ErrorResp {
//         return BadRequest(Json(MessageResp { message: err }));
//     }
// }

pub type AppResult<T> = Result<T, ErrorResp>;

pub type JsonResult<T> = AppResult<Json<T>>;

pub fn bad_req(err: String) -> ErrorResp {
    BadRequest(Json(MessageResp { message: err }))
}

pub fn unwrap_bad_req<T, E>(x: Result<T, E>) -> Result<T, ErrorResp>
where
    E: Display,
{
    match x {
        Ok(x) => Ok(x),
        Err(err) => Err(bad_req(err.to_string())),
    }
}

#[derive(Serialize)]
pub struct DNA {
    pub location: dna::Location,
    pub dna: String,
}

#[derive(Serialize)]
pub struct DNAResp {
    pub assembly: String,
    pub seqs: Vec<DNA>,
}

#[derive(Serialize)]
pub struct DNAJsonResp {
    pub data: DNAResp,
}

// pub fn parse_loc_from_route(
//     chr: Option<&str>,
//     start: Option<u32>,
//     end: Option<u32>,
//     default_chr: &str,
//     default_start: u32,
//     default_end: u32,
// ) -> Result<dna::Location, String> {
//     let c: &str = match chr {
//         Some(c) => c,
//         None => default_chr,
//     };

//     let s: u32 = match start {
//         Some(s) => s,
//         None => default_start,
//     };

//     let e: u32 = match end {
//         Some(e) => e,
//         None => default_end,
//     };

//     let loc: Location = Location::new(c, s, e)?;

//     Ok(loc)
// }

// pub fn parse_assembly_from_route(assembly: Option<&str>) -> String {
//     let a: &str = match assembly {
//         Some(assembly) => assembly,
//         None => "grch38",
//     };

//     return a.to_string();
// }

pub fn parse_bool(b: &str) -> bool {
    match b {
        "true" => true,
        "t" => true,
        "false" => false,
        "f" => false,
        _ => false,
    }
}

pub fn parse_level_from_route(level: Option<&str>) -> Level {
    return match level {
        Some(l) => Level::from(l),
        None => Level::Gene,
    };
}

pub fn parse_closest_n_from_route(n: Option<u16>) -> u16 {
    return match n {
        Some(nn) => nn,
        None => 5,
    };
}

pub fn parse_tss_from_query(tss: Option<&str>) -> TSSRegion {
    return match tss {
        Some(ts) => {
            let tokens: Vec<&str> = ts.split(",").collect();

            let s: u32 = match tokens[0].parse::<u32>() {
                Ok(s) => s,
                Err(_) => return TSSRegion::default(),
            };

            let e: u32 = match tokens[1].parse::<u32>() {
                Ok(s) => s,
                Err(_) => return TSSRegion::default(),
            };

            TSSRegion::new(s, e)
        }
        None => TSSRegion::default(),
    };
}

pub fn parse_output_from_query(output: Option<&str>) -> String {
    return match output {
        Some(output) => {
            if output == "text" {
                "text".to_owned()
            } else {
                "json".to_owned()
            }
        }
        None => "json".to_owned(),
    };
}

pub fn create_genesdb(assembly: &str) -> Result<Loctogene, Box<dyn Error>> {
    return Loctogene::new(&format!("data/loctogene/{}.db", assembly));
}

pub fn create_userdb() -> AuthResult<UserDb> {
    return UserDb::new("data/users.db");
}
