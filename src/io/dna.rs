use dna::{DnaSeq, Location};
use serde::{Deserialize, Serialize};

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
