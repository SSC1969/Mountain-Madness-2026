use ratatui::{
    style::{Color, Stylize},
    text::{Line, Span, Text},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use strum::{EnumCount, EnumProperty, VariantArray};

use crate::{
    inventory::Inventory,
    items::{
        Item, ItemTypes,
        fish::{FishQuality, SPECIES, Species, SpeciesRef},
        rod::{RODS, Rod},
    },
};

#[derive(Serialize, Deserialize)]
pub struct Dex {
    items: HashMap<String, DexEntries>,
}

impl Dex {
    pub fn get(&self, name: String) -> Option<&DexEntries> {
        self.items.get(&name)
    }

    pub fn get_mut(&mut self, name: String) -> Option<&mut DexEntries> {
        self.items.get_mut(&name)
    }

    pub fn get_all(&self) -> Vec<&DexEntries> {
        self.items.values().collect()
    }

    pub fn try_create(&mut self, species: &'static Species) {
        self.items.insert(
            species.name.clone(),
            DexEntries::Fish(FishEntry::new(species)),
        );
    }
}

impl Default for Dex {
    fn default() -> Self {
        let mut items: HashMap<String, DexEntries> = HashMap::new();
        for spec in SPECIES.iter() {
            items.insert(spec.name.clone(), DexEntries::Fish(FishEntry::new(spec)));
        }
        for rod in RODS.iter() {
            items.insert(rod.name.clone(), DexEntries::Rod(rod.clone()));
        }
        Self { items }
    }
}

impl Inventory for Dex {
    fn add_item(&mut self, item: ItemTypes) {
        if let Some(entry) = self.get_mut(item.name()) {
            entry.update(item);
        } else {
            match item {
                ItemTypes::Fish(fish) => {
                    self.items.insert(
                        fish.species.0.name.clone(),
                        DexEntries::Fish(FishEntry::new(fish.species.0)),
                    );
                }
                ItemTypes::Rod(rod) => {
                    self.items
                        .insert(rod.name.clone(), DexEntries::Rod(rod.clone()));
                }
            }
        }
    }
    fn remove_item(&mut self, item: ItemTypes) {
        self.items.remove(&item.name());
    }
}

pub trait DexEntry {
    /// Updates this entry based on the newly passed in item
    fn update(&mut self, item: ItemTypes);
    /// Gets the display text for this entry
    fn get_lines(&self) -> Text<'_>;
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum DexEntries {
    Fish(FishEntry),
    Rod(Rod),
}

impl DexEntry for DexEntries {
    fn update(&mut self, item: ItemTypes) {
        match self {
            DexEntries::Fish(entry) => entry.update(item),
            DexEntries::Rod(entry) => entry.update(item),
        }
    }

    fn get_lines(&self) -> Text<'_> {
        match self {
            DexEntries::Fish(entry) => entry.get_lines(),
            DexEntries::Rod(entry) => entry.get_lines(),
        }
    }
}

impl PartialOrd for DexEntries {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            DexEntries::Fish(fish) => {
                if let DexEntries::Fish(o_fish) = other {
                    Some(
                        fish.species
                            .0
                            .rarity
                            .cmp(&o_fish.species.0.rarity)
                            .then(fish.species.0.name.cmp(&o_fish.species.0.name)),
                    )
                } else {
                    None
                }
            }
            DexEntries::Rod(rod) => {
                if let DexEntries::Rod(o_rod) = other {
                    Some(rod.name.cmp(&o_rod.name))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct FishEntry {
    species: SpeciesRef,
    count: u32,
    total_value: i32,
    highest_value: i32,
    largest: f32,
    heaviest: f32,
    qualities: [bool; FishQuality::COUNT],
}

impl FishEntry {
    fn new(species: &'static Species) -> Self {
        Self {
            species: SpeciesRef(species),
            count: 0,
            total_value: 0,
            highest_value: 0,
            largest: 0.0,
            heaviest: 0.0,
            qualities: [false; FishQuality::COUNT],
        }
    }
}

impl DexEntry for FishEntry {
    fn update(&mut self, item: ItemTypes) {
        if let ItemTypes::Fish(fish) = item {
            self.count += 1;
            self.total_value += fish.value();
            self.highest_value = i32::max(self.highest_value, fish.value());
            self.largest = f32::max(self.largest, fish.length);
            self.heaviest = f32::max(self.heaviest, fish.weight);

            self.qualities[fish.quality as usize] = true;
        }
    }

    fn get_lines(&self) -> Text<'_> {
        Text::from({
            if self.count <= 0 {
                vec![
                    Line::from("???").bold().underlined(),
                    "Not discovered yet!".into(),
                ]
            } else {
                let mut vec = self.species.0.icon();
                vec.extend([" ".into(), self.species.0.name.clone().into()]);
                vec = vec.into_iter().map(|i| i.bold().underlined()).collect();
                vec.push(" ".into());
                vec.extend(self.qualities.iter().enumerate().map(|(i, &q)| {
                    let s = Span::from("*").not_underlined().not_bold();
                    if q {
                        s.fg(
                            Color::from_str(FishQuality::VARIANTS[i].get_str("color").unwrap())
                                .unwrap(),
                        )
                    } else {
                        s.dim()
                    }
                }));
                let l1 = Line::from(vec);

                vec = vec![
                    format!("Caught: {}(${})", self.count, self.total_value,).into(),
                    " ".into(),
                ];
                let l2 = Line::from(vec);

                let l3 = Line::from(vec![
                    format!(
                        "Best: {:.1}cm, {:.1}kg, ${}",
                        self.largest, self.heaviest, self.highest_value
                    )
                    .into(),
                ]);

                vec![l1, l2, l3]
            }
        })
    }
}

impl DexEntry for Rod {
    fn update(&mut self, item: ItemTypes) {
        if let ItemTypes::Rod(rod) = item {
            *self = Rod { ..rod };
        }
    }

    fn get_lines(&self) -> Text<'_> {
        Text::from({
            let mut vec = self.icon();
            vec.extend([" ".into(), self.name().into()]);
            let l1 = Line::from(vec).bold().underlined();

            let l2 = Line::from(format!(
                "Lure: {} | Hook: {}",
                self.lure_mult, self.hook_strength
            ));

            vec![l1, l2]
        })
    }
}
