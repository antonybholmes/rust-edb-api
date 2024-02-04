pub fn parse_loc_from_route(
    chr: Option<&str>,
    start: Option<u32>,
    end: Option<u32>,
    default_chr: &str,
    default_start: u32,
    default_end: u32,
) -> Result<dna::Location, String> {
    let c = match chr {
        Some(c) => c,
        None => default_chr,
    };

    let s = match start {
        Some(s) => s,
        None => default_start,
    };

    let e = match end {
        Some(e) => e,
        None => default_end,
    };

    let loc: dna::Location = match dna::Location::new(c, s, e) {
        Ok(loc) => loc,
        Err(err) => return Err(err),
    };

    Ok(loc)
}