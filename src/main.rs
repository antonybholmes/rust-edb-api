fn main() {
    println!("Hello, world!");

    let loc = dna::Location::parse("chr1:100000-100100");

    println!("{}", loc);

    let dir:&str="/ifs/scratch/cancer/Lab_RDF/ngs/dna/hg19" ;

    let dna_db : dna::DNA = dna::DNA::new(dir);

    let dna = dna_db.get_dna(&loc, true, true);

    println!("{}",dna);
}
