use std::sync::LazyLock;

use rand::RngExt;
use rand_distr::Distribution;
use rand_distr::weighted::WeightedIndex;
use ratatui::{
    style::{Color, Style},
    text::Span,
};
use serde::Deserialize;
use strum::{EnumIter, EnumProperty, IntoEnumIterator, VariantArray};

use crate::items::Item;

// include the species .json file in the compiled binary
const SPECIES_JSON: &str = include_str!("species.json");

pub static SPECIES: LazyLock<Vec<Species>> = LazyLock::new({
    || serde_json::from_str(&SPECIES_JSON).expect("Error deserializing species!")
});

//TODO: convert from u32 to float
#[derive(Default, Eq, PartialEq, Clone, Debug)]
pub struct Fish {
    pub species: Species,
    pub length: u32,
    pub weight: u32,
    pub quality: FishQuality,
}

#[derive(Default, Deserialize, Debug, Eq, PartialEq, Hash, Clone, Copy, EnumIter)]
pub enum SpeciesRarity {
    #[default]
    Common,
    Rare,
    Epic,
    Legendary,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, VariantArray, EnumProperty, EnumIter)]
pub enum FishQuality {
    #[strum(props(w = 50))]
    Shoddy,
    #[strum(props(w = 40))]
    Mediocre,
    #[strum(props(w = 30))]
    Average,
    #[strum(props(w = 10))]
    Fine,
    #[strum(props(w = 5))]
    Lovely,
    #[strum(props(w = 1))]
    Resplendent,
}

impl Fish {
    pub fn generate() -> Self {
        let mut rng = rand::rng();
        let s = SPECIES[rng.random_range(0..SPECIES.len())].clone();
        let length = rng.random_range(s.min_len..s.max_len);
        let weight = rng.random_range(s.min_weight..s.max_weight);
        let quality = FishQuality::generate();

        return Fish {
            species: s,
            length,
            weight,
            quality,
        };
    }
}

impl Item for Fish {
    fn name(&self) -> String {
        self.species.name.clone()
    }

    fn value(&self) -> i32 {
        let species = &self.species;
        let weight_factor = (self.weight - species.min_weight) as f32
            / (species.max_weight - species.min_weight) as f32;
        (species.base_value as f32 * species.rarity.multiplier() * (weight_factor as f32 + 0.5))
            as i32
    }

    fn info(&self) -> String {
        format!("{}g | {}cm - {:?}", self.weight, self.length, self.quality)
    }

    fn icon(&self) -> Vec<Span<'_>> {
        vec![self.species.icon()]
    }
}

impl FishQuality {
    fn generate() -> Self {
        let mut rng = rand::rng();
        let qualities: Vec<FishQuality> = FishQuality::VARIANTS.to_vec();
        let weights: Vec<i64> = FishQuality::iter()
            .map(|q| q.get_int("w").unwrap())
            .collect();
        let dist = WeightedIndex::new(&weights).unwrap();

        qualities[dist.sample(&mut rng)].clone()
    }
}

impl Default for FishQuality {
    fn default() -> Self {
        FishQuality::generate()
    }
}

impl SpeciesRarity {
    pub fn multiplier(&self) -> f32 {
        match self {
            SpeciesRarity::Common => 1.0,
            SpeciesRarity::Rare => 1.5,
            SpeciesRarity::Epic => 2.0,
            SpeciesRarity::Legendary => 2.5,
        }
    }

    pub fn odds(&self) -> f32 {
        match self {
            SpeciesRarity::Common => 0.5,
            SpeciesRarity::Rare => 0.3,
            SpeciesRarity::Epic => 0.15,
            SpeciesRarity::Legendary => 0.05,
        }
    }
}

#[derive(PartialEq, Eq, Deserialize, Default, Debug, Hash, Clone)]
pub struct Species {
    pub name: String,
    pub base_value: u32,
    pub min_len: u32,
    pub max_len: u32,
    pub min_weight: u32,
    pub max_weight: u32,
    pub icon: String,
    pub colour: Color,
    pub rarity: SpeciesRarity,
}

impl Species {
    pub fn icon(&self) -> Span<'_> {
        Span::styled(self.icon.clone(), Style::default().fg(self.colour))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    // fn test_load_species() {
    //     assert!(species.is_ok());

    //     let species = species.unwrap();

    //     println!("{} species read\n", species.len());
    //     for s in &species {
    //         println!(
    //             "{}: icon {} len {}–{}cm, weight {}–{}kg, {:?}, {:?}",
    //             s.name,
    //             s.icon,
    //             s.min_len,
    //             s.max_len,
    //             s.min_weight,
    //             s.max_weight,
    //             s.rarity,
    //             s.colour
    //         );
    //     }
    //     println!("");
    // }
    #[test]
    fn test_generate_fish() {
        for _ in 0..10 {
            let fish = Fish::generate();
            println!("{:?}", fish);
        }
    }
}
