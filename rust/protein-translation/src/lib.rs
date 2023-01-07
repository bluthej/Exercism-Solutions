use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    conversion_table: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.conversion_table.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|chunk| self.name_for(String::from_iter(chunk).as_ref()))
            .take_while(|&protein| protein != Some("stop codon"))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let conversion_table = pairs.into_iter().collect();
    CodonsInfo { conversion_table }
}
