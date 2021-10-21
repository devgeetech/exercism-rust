pub struct Allergies {
    score: u32
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts  = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

const ALLERGEN_LIST: [Allergen; 8] = [Allergen::Eggs, Allergen::Peanuts, Allergen::Shellfish, Allergen::Strawberries, Allergen::Tomatoes, Allergen::Chocolate, Allergen::Pollen, Allergen::Cats];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            score: score
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen = *allergen as u32;
        self.score & allergen == allergen
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let allergens = ALLERGEN_LIST
            .iter()
            .filter(|x| self.is_allergic_to(x))
            .map(|&y| y)
            .collect();
        allergens
    }
}
