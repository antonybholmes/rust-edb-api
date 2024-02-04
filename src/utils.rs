pub fn parse_loc_from_query(
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

    let loc: dna::Location = match dna::Location::new(c, s, e) {
        Ok(loc) => loc,
        Err(err) => return Err(err),
    };

    Ok(loc)
}

pub fn parse_assembly_from_query(assembly: Option<&str>) -> String {
    let a: &str = match assembly {
        Some(assembly) => assembly,
        None => "grch38",
    };

    return a.to_string();
}
