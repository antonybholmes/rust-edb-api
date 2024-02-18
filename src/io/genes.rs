use csv::WriterBuilder;
use dna::Location;
use genes::{annotate::{Annotate, ClosestGene, GeneAnnotation}, loctogene::{GenesResult, GenomicFeature, Level, LoctogeneDb, TSSRegion}};
use rocket::serde::json::Json;
use serde::Serialize;
use serde_json::json;

use super::dna::DnaBody;
 
#[derive(Serialize)]
pub struct LocationGenes {
    pub location: Location,
    pub features: Vec<GenomicFeature>,
}

#[derive(Serialize)]
pub struct GenesJsonData {
    pub level: Level,
    pub features: Vec<LocationGenes>,
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
pub struct AnnotationJsonResp {
    pub data: GeneAnnotationTable,
}

pub fn create_genesdb(assembly: &str) -> GenesResult<LoctogeneDb> {
   return LoctogeneDb::new(&format!("data/loctogene/{}.db", assembly));
}

pub fn parse_level_from_route(level: Option<&str>) -> Level {
    return match level {
        Some(l) => Level::from(l),
        None => Level::Gene,
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

pub fn make_gene_json(
    annotatedb: &Annotate,
    body: &Json<DnaBody>,
    closest_n: u16,
) -> GenesResult<String> {
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
    while table.closest_genes.len() < closest_n as usize {
        table.closest_genes.push(Vec::with_capacity(l));
    }

    for location in body.locations.iter() {
        let annotation: GeneAnnotation = annotatedb.annotate(&location)?;

        table.location.push(location.clone());
        table.gene_ids.push(annotation.gene_ids);
        table.gene_symbols.push(annotation.gene_symbols);
        table.prom_labels.push(annotation.prom_labels);
        table.tss_dists.push(annotation.tss_dists);

        for (ci, closest_gene) in annotation.closest_genes.iter().enumerate() {
            table
                .closest_genes
                .get_mut(ci)
                .unwrap()
                .push(closest_gene.clone());
        }
    }

    let d: String = json!(AnnotationJsonResp { data: table }).to_string();

    return Ok(d);
}

pub fn make_gene_table(
    annotatedb: &Annotate,
    body: &Json<DnaBody>,
    closest_n: u16,
    ts: &TSSRegion,
) -> GenesResult<String> {
    let mut wtr: csv::Writer<Vec<u8>> = WriterBuilder::new().delimiter(b'\t').from_writer(vec![]);

    let capacity: usize = 6 + closest_n as usize;

    let mut headers: Vec<String> = Vec::with_capacity(capacity);

    headers.push("Location".to_owned());
    headers.push("ID".to_owned());
    headers.push("Gene Symbol".to_owned());
    headers.push(format!(
        "Relative To Gene (prom=-{}/+{}kb)",
        ts.offset_5p() / 1000,
        ts.offset_3p() / 1000
    ));
    headers.push("TSS Distance".to_owned());

    for i in 1..(closest_n + 1) {
        headers.push(format!("#{} Closest ID", i));
        headers.push(format!("#{} Closest Gene Symbols", i));
        headers.push(format!(
            "#{} Relative To Closet Gene (prom=-{}/+{}kb)",
            i,
            ts.offset_5p() / 1000,
            ts.offset_3p() / 1000
        ));
        headers.push(format!("#{} TSS Closest Distance", i));
    }

    wtr.write_record(&headers)?;

    for location in body.locations.iter() {
        let annotation: GeneAnnotation = annotatedb.annotate(&location)?;

        let mut row: Vec<String> = Vec::with_capacity(capacity);

        row.push(location.to_string());
        row.push(annotation.gene_ids);
        row.push(annotation.gene_symbols);
        row.push(annotation.prom_labels);
        row.push(annotation.tss_dists);

        for closest_gene in annotation.closest_genes.iter() {
            row.push(closest_gene.gene_id.clone());
            row.push(closest_gene.gene_symbol.clone());
            row.push(closest_gene.prom_label.clone());
            row.push(closest_gene.tss_dist.to_string());
        }

        wtr.write_record(&row)?;
    }

    let inner: Vec<u8> = wtr.into_inner()?;
    let data: String = String::from_utf8(inner)?;

    Ok(data)
}
