use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DnaBody {
    pub locations: Vec<dna::Location>,
}


#[derive(Serialize)]
pub struct DnaResp {
    pub assembly: String,
    pub seqs: Vec<dna::DNA>,
}

#[derive(Serialize)]
pub struct DnaJsonResp {
    pub data: DnaResp,
}