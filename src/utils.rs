pub mod genes;

use std::error::Error;

use dna::Location;
use loctogene::{Level, Loctogene, TSSRegion};
use serde::{Deserialize, Serialize};

#[macro_export]
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

#[derive(Serialize)]
pub struct DNAJsonData {
    pub location: String,
    pub dna: String,
}

#[derive(Serialize)]
pub struct DNAJsonResp {
    pub data: DNAJsonData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnotationBody {
    locations: Vec<Location>,
}

pub fn parse_loc_from_route(
    chr: Option<&str>,
    start: Option<u32>,
    end: Option<u32>,
    default_chr: &str,
    default_start: u32,
    default_end: u32,
) -> Result<dna::Location, String> {
    let c: &str = match chr {
        Some(c) => c,
        None => default_chr,
    };

    let s: u32 = match start {
        Some(s) => s,
        None => default_start,
    };

    let e: u32 = match end {
        Some(e) => e,
        None => default_end,
    };

    let loc: Location = Location::new(c, s, e)?;

    Ok(loc)
}

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
