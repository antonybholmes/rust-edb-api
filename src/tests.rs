#[cfg(test)]


#[test]
fn test_dna()  {
 
    let res: Result<dna::Location, String> = dna::Location::parse("chr1:100000-100100");

    assert!(!res.is_err());

    let loc = res.unwrap();

    println!("{}", loc);

 
    let dna_db: dna::DNA = dna::DNA::new("/ifs/scratch/cancer/Lab_RDF/ngs/dna/hg19".to_string());

    let res = dna_db.get_dna(&loc, true, true);

    assert!(!res.is_err());

    let dna: String = res.unwrap();

    println!("{}", dna);
}
