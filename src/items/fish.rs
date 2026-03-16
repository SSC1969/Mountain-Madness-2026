use std::{str::FromStr, sync::LazyLock};

use rand_distr::weighted::WeightedIndex;
use rand_distr::{Distribution, Normal};
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
use strum::{EnumCount, EnumIter, EnumProperty, IntoEnumIterator, VariantArray};

use crate::items::Item;

// include the species .json file in the compiled binary
const SPECIES_JSON: &str = include_str!("species.json");

pub static SPECIES: LazyLock<Vec<Species>> = LazyLock::new({
    || serde_json::from_str(&SPECIES_JSON).expect("Error deserializing species!")
});

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

        // generate a fish based on their rarity
        let weights: Vec<f32> = SPECIES.iter().map(|s| s.rarity.odds()).collect();
        let dist = WeightedIndex::new(&weights).unwrap();
        let s = &SPECIES[dist.sample(&mut rng)];

        // generate length/weight based on a normal distribution
        let len_norm = Normal::new(1.25 * s.avg_len, 0.55 * s.avg_len).unwrap();
        let length = len_norm.sample(&mut rng).abs();
        let weight_norm = Normal::new(1.25 * s.avg_weight, 0.55 * s.avg_weight).unwrap();
        let weight = weight_norm.sample(&mut rng).abs();

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
        let weight_factor = self.weight / species.0.avg_weight;
        let length_factor = self.length / species.0.avg_len;
        let size_factor = (weight_factor + length_factor) / 2.0;

        let size_mult = {
            if size_factor <= 0.25 {
                1.75
            } else if size_factor <= 0.5 {
                0.6
            } else if size_factor <= 1.0 {
                0.8
            } else if size_factor <= 1.5 {
                1.0
            } else if size_factor <= 2.0 {
                1.5
            } else if size_factor <= 3.0 {
                2.5
            } else {
                4.25
            }
        };

        (species.0.value() as f32 * (self.quality.get_int("v").unwrap() as f32 / 10.0) * size_mult)
            as i32
    }

    fn info(&'_ self) -> Line<'_> {
        Line::from(vec![
            format!("{:.1}kg | {:.1}cm - ", self.weight, self.length).into(),
            Span::from(format!("{:?}", self.quality))
                .fg(Color::from_str(self.quality.get_str("color").unwrap()).unwrap()),
        ])
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
    pub avg_len: f32,
    pub avg_weight: f32,
    pub icon: Vec<(String, Style)>,
    pub rarity: SpeciesRarity,
}

impl Species {
    pub fn icon(&self) -> Vec<Span<'_>> {
        self.icon.iter().map(|(p, s)| Span::styled(p, *s)).collect()
    }

    pub fn value(&self) -> i32 {
        self.rarity.multiplier()
    }
}

#[derive(
    VariantArray,
    Default,
    Deserialize,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Clone,
    Copy,
    EnumIter,
    Serialize,
    PartialOrd,
    Ord,
)]
pub enum SpeciesRarity {
    #[default]
    Common,
    Rare,
    Epic,
    Legendary,
}

impl SpeciesRarity {
    pub fn multiplier(&self) -> i32 {
        match self {
            SpeciesRarity::Common => 50,
            SpeciesRarity::Rare => 150,
            SpeciesRarity::Epic => 500,
            SpeciesRarity::Legendary => 2000,
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
    PartialEq,
    Eq,
    Debug,
    Hash,
    Clone,
    VariantArray,
    EnumProperty,
    EnumIter,
    Serialize,
    Deserialize,
    EnumCount,
)]
pub enum FishQuality {
    #[strum(props(w = 1250, v = 3, color = "#946851"))]
    Shoddy,
    #[strum(props(w = 750, v = 5, color = "#8AC944"))]
    Mediocre,
    #[strum(props(w = 350, v = 10, color = "#44C7C9"))]
    Average,
    #[strum(props(w = 15, v = 50, color = "#b9286e"))]
    Lovely,
    #[strum(props(w = 1, v = 100, color = "#C6A9DE"))]
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

    #[test]
    fn test_generate_fish() {
        for _ in 0..10 {
            let fish = Fish::generate();
            println!("{:?}", fish);
        }
    }
}
