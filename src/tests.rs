#[cfg(test)]

#[test]
fn test_dna() {
    let res: Result<dna::Location, String> = dna::Location::parse("chr1:100000-100100");

    //assert!(!res.is_err());

    let loc = res.unwrap();

    println!("{}", loc);

    let dna_db: dna::DNA = dna::DNA::new("data/dna/hg19".to_string());

    let res = dna_db.get_dna(
        &loc,
        true,
        true,
        &dna::CaseMask::None,
        &dna::RepeatMask::None,
    );

    assert!(!res.is_err());

    let dna: String = res.unwrap();

    println!("test {}", dna);
}

#[test]
fn test_uc_dna() {
    let res: Result<dna::Location, String> = dna::Location::parse("chr1:100000-100100");

    //assert!(!res.is_err());

    let loc = res.unwrap();

    println!("{}", loc);

    let dna_db: dna::DNA = dna::DNA::new("data/dna/hg19".to_string());

    let res = dna_db.get_dna(
        &loc,
        true,
        true,
        &dna::CaseMask::Upper,
        &dna::RepeatMask::None,
    );

    assert!(!res.is_err());

    let dna: String = res.unwrap();

    println!("test {}", dna);
}

#[test]
fn test_rep_mask_n_dna() {
    let res: Result<dna::Location, String> = dna::Location::parse("chr1:100000-100100");

    //assert!(!res.is_err());

    let loc = res.unwrap();

    println!("{}", loc);

    let dna_db: dna::DNA = dna::DNA::new("data/dna/hg19".to_string());

    let res = dna_db.get_dna(
        &loc,
        true,
        true,
        &dna::CaseMask::Upper,
        &dna::RepeatMask::N,
    );

    assert!(!res.is_err());

    let dna: String = res.unwrap();

    println!("test {}", dna);
}

#[test]
fn test_rep_mask_lower_dna() {
    let res: Result<dna::Location, String> = dna::Location::parse("chr1:100000-100100");

    //assert!(!res.is_err());

    let loc = res.unwrap();

    println!("{}", loc);

    let dna_db: dna::DNA = dna::DNA::new("data/dna/hg19".to_string());

    let res = dna_db.get_dna(
        &loc,
        true,
        true,
        &dna::CaseMask::Upper,
        &dna::RepeatMask::Lower,
    );

    assert!(!res.is_err());

    let dna: String = res.unwrap();

    println!("test {}", dna);
}