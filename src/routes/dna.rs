use dna::{DnaDb, DnaSeq, Format, Location, RepeatMask};
use serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

use auth::{
    jwt::Jwt,
    AuthResult,
};

use crate::utils::{parse_bool, unwrap_bad_req, JsonResult};

#[derive(Serialize, Deserialize)]
pub struct DnaBody {
    pub locations: Vec<Location>,
}

#[derive(Serialize)]
pub struct DnaResp {
    pub assembly: String,
    pub seqs: Vec<DnaSeq>,
}

#[derive(Serialize)]
pub struct DnaJsonResp {
    pub data: DnaResp,
}

#[post(
    "/<assembly>?<format>&<mask>&<rev>&<comp>",
    format = "application/json",
    data = "<data>"
)]
pub fn dna_route(
    assembly: &str,
    rev: Option<&str>,
    comp: Option<&str>,
    format: Option<&str>,
    mask: Option<&str>,
    data: Json<DnaBody>,
    jwt: AuthResult<Jwt>,
) -> JsonResult<DnaJsonResp> {
    // test if key valid
    let _key = unwrap_bad_req(jwt)?;

    let loc = data.locations.get(0).unwrap();

    let r = match rev {
        Some(r) => parse_bool(r),
        None => false,
    };

    let rc = match comp {
        Some(rc) => parse_bool(rc),
        None => false,
    };

    let format = match format {
        Some(rc) => match rc {
            "lower" => Format::Lower,
            "upper" => Format::Upper,
            _ => Format::None,
        },
        None => Format::None,
    };

    let repeat_mask = match mask {
        Some(rc) => match rc {
            "lower" => RepeatMask::Lower,
            "n" => RepeatMask::N,
            _ => RepeatMask::None,
        },
        None => RepeatMask::None,
    };

    let dna_db = DnaDb::new(&format!("data/dna/{}", assembly));

    let mut seqs = Vec::with_capacity(data.locations.len());

    for location in data.locations.iter() {
        let dna: String = unwrap_bad_req(dna_db.dna(&loc, r, rc, &format, &repeat_mask))?;

        seqs.push(dna::DnaSeq {
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