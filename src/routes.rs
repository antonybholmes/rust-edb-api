pub mod dna;
pub mod genes;
pub mod users;

use std::fmt::Display;



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

pub fn parse_closest_n_from_route(n: Option<u16>) -> u16 {
    return match n {
        Some(nn) => nn,
        None => 5,
    };
}

pub fn parse_output_from_query(output: Option<&str>) -> String {
    return match output {
        Some(output) => {
            if output == "text" {
                "text".to_string()
            } else {
                "json".to_string()
            }
        }
        None => "json".to_string(),
    };
}


