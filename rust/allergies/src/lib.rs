pub struct Allergies {
    list: Vec<bool>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let list = format!("{:08b}", score)
            .chars()
            .rev()
            .map(|b| b == '1')
            .collect();
        Allergies { list }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.list[*allergen as usize]
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (0..8)
            .map(|i| match i {
                0 => Allergen::Eggs,
                1 => Allergen::Peanuts,
                2 => Allergen::Shellfish,
                3 => Allergen::Strawberries,
                4 => Allergen::Tomatoes,
                5 => Allergen::Chocolate,
                6 => Allergen::Pollen,
                7 => Allergen::Cats,
                _ => unreachable!(),
            })
            .filter(|a| self.is_allergic_to(a))
            .collect()
    }
}
