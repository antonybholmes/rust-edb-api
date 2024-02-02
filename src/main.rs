
#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, json::Json};
 
 
mod tests;

#[derive(Serialize)]
pub struct Message {
    pub message: String,
}

#[derive(Serialize)]
pub struct DNA {
    pub location: String,
    pub dna: String
}

#[get("/")]
fn index() -> Json<Message> {
    Json(Message{message:"cake".to_string()})
}

#[get("/dna")]
fn dna_route() -> Json<DNA>{
    let loc = dna::Location::parse("chr1:100000-100100");

    println!("{}", loc);

    let dir:&str="/ifs/scratch/cancer/Lab_RDF/ngs/dna/hg19";

    let dna_db : dna::DNA = dna::DNA::new(dir);

    let dna = dna_db.get_dna(&loc, true, true);

    return Json(DNA{location:loc.to_string(), dna})
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount("/v1", routes![dna_route])
}
