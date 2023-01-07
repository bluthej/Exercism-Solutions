#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna.chars().position(|c| !"ACGT".contains(c)) {
            Some(i) => Err(i),
            None => Ok(Dna(dna.to_string())),
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|c| match c {
                'G' => "C",
                'C' => "G",
                'T' => "A",
                'A' => "U",
                _ => unreachable!(),
            })
            .collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna.chars().position(|c| !"ACGU".contains(c)) {
            Some(i) => Err(i),
            None => Ok(Rna(rna.to_string())),
        }
    }
}
