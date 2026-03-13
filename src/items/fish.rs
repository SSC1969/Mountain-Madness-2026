use std::sync::LazyLock;

use rand::RngExt;
use rand_distr::Distribution;
use rand_distr::weighted::WeightedIndex;
use ratatui::{style::Style, text::Span};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
use strum::{EnumIter, EnumProperty, IntoEnumIterator, VariantArray};

use crate::items::Item;

// include the species .json file in the compiled binary
const SPECIES_JSON: &str = include_str!("species.json");

pub static SPECIES: LazyLock<Vec<Species>> = LazyLock::new({
    || serde_json::from_str(&SPECIES_JSON).expect("Error deserializing species!")
});

//TODO: convert from u32 to float
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Fish {
    pub species: SpeciesRef,
    pub length: f32,
    pub weight: f32,
    pub quality: FishQuality,
}

impl Fish {
    pub fn generate() -> Self {
        let mut rng = rand::rng();
        let s = &SPECIES[rng.random_range(0..SPECIES.len())];
        let length = rng.random_range(s.min_len..s.max_len);
        let weight = rng.random_range(s.min_weight..s.max_weight);
        let quality = FishQuality::generate();

        return Fish {
            species: SpeciesRef(s),
            length,
            weight,
            quality,
        };
    }
}

impl Item for Fish {
    fn name(&self) -> String {
        self.species.0.name.clone()
    }

    fn value(&self) -> i32 {
        let species = &self.species;
        let weight_factor = (self.weight - species.0.min_weight) as f32
            / (species.0.max_weight - species.0.min_weight) as f32;
        (species.0.base_value as f32 * species.0.rarity.multiplier() * (weight_factor as f32 + 0.5))
            as i32
    }

    fn info(&self) -> String {
        format!(
            "{:.1}kg | {:.1}cm - {:?}",
            self.weight, self.length, self.quality
        )
    }

    fn icon(&self) -> Vec<Span<'_>> {
        self.species.0.icon()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct SpeciesRef(pub &'static Species);

impl<'de> Deserialize<'de> for SpeciesRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match SPECIES.iter().find(|&spec| spec.name == s) {
            Some(spec) => Ok(SpeciesRef(spec)),
            None => Err(D::Error::custom("Error deserializing species!")),
        }
    }
}

impl Serialize for SpeciesRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.name)
    }
}

#[derive(PartialEq, Deserialize, Serialize, Default, Debug, Clone)]
pub struct Species {
    pub name: String,
    pub base_value: u32,
    pub min_len: f32,
    pub max_len: f32,
    pub min_weight: f32,
    pub max_weight: f32,
    pub icon: Vec<(String, Style)>,
    pub rarity: SpeciesRarity,
}

impl Species {
    pub fn icon(&self) -> Vec<Span<'_>> {
        self.icon.iter().map(|(p, s)| Span::styled(p, *s)).collect()
    }
}

#[derive(Default, Deserialize, Debug, Eq, PartialEq, Hash, Clone, Copy, EnumIter, Serialize)]
pub enum SpeciesRarity {
    #[default]
    Common,
    Rare,
    Epic,
    Legendary,
}

impl SpeciesRarity {
    pub fn multiplier(&self) -> f32 {
        match self {
            SpeciesRarity::Common => 1.0,
            SpeciesRarity::Rare => 2.0,
            SpeciesRarity::Epic => 5.0,
            SpeciesRarity::Legendary => 10.0,
        }
    }

    pub fn odds(&self) -> f32 {
        match self {
            SpeciesRarity::Common => 1.0,
            SpeciesRarity::Rare => 0.3,
            SpeciesRarity::Epic => 0.05,
            SpeciesRarity::Legendary => 0.001,
        }
    }
}

#[derive(
    PartialEq, Eq, Debug, Hash, Clone, VariantArray, EnumProperty, EnumIter, Serialize, Deserialize,
)]
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
