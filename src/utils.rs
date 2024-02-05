use loctogene::Loctogene;

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

    let loc: dna::Location = match dna::Location::new(c, s, e) {
        Ok(loc) => loc,
        Err(err) => return Err(err),
    };

    Ok(loc)
}

pub fn parse_assembly_from_route(assembly: Option<&str>) -> String {
    let a: &str = match assembly {
        Some(assembly) => assembly,
        None => "grch38",
    };

    return a.to_string();
}

pub fn parse_bool(b: &str) -> bool {
    match b {
        "true" => true,
        "t" => true,
        "false" => false,
        "f" => false,
        _ => false,
    }
}

pub fn parse_level_from_route(level:Option<&str>) -> loctogene::Level {
    return match level {
        Some(l) => loctogene::Level::from(l),
        None => loctogene::Level::Gene,
    };
}

 

pub fn parse_closest_n_from_route(n:Option<u16>) -> u16 {
    return match n {
        Some(nn) => nn,
        None => 10,
    };
}

pub fn create_genesdb(assembly:&str) -> Result<Loctogene, String>{
    return loctogene::Loctogene::new(&format!("data/loctogene/{}.db", assembly));
}

 